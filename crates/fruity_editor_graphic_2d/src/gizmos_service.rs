use fruity_game_engine::any::FruityAny;
use fruity_game_engine::export_impl;
use fruity_game_engine::export_struct;
use fruity_game_engine::resource::resource_container::ResourceContainer;
use fruity_game_engine::resource::resource_reference::ResourceReference;
use fruity_game_engine::FruityResult;
use fruity_graphic::graphic_service::GraphicService;
use fruity_graphic::math::matrix3::Matrix3;
use fruity_graphic::math::matrix4::Matrix4;
use fruity_graphic::math::vector2d::Vector2D;
use fruity_graphic::math::Color;
use fruity_graphic_2d::graphic_2d_service::Graphic2dService;
use fruity_input::drag_service::DragCallback;
use fruity_input::drag_service::DragEndCallback;
use fruity_input::drag_service::DragService;
use fruity_input::input_service::InputService;
use std::fmt::Debug;

#[derive(Debug, FruityAny)]
#[export_struct]
pub struct GizmosService {
    input_service: ResourceReference<InputService>,
    drag_service: ResourceReference<DragService>,
    graphic_service: ResourceReference<dyn GraphicService>,
    graphic_2d_service: ResourceReference<Graphic2dService>,
}

#[export_impl]
impl GizmosService {
    pub fn new(resource_container: ResourceContainer) -> GizmosService {
        let input_service = resource_container.require::<InputService>();
        let drag_service = resource_container.require::<DragService>();
        let graphic_service = resource_container.require::<dyn GraphicService>();
        let graphic_2d_service = resource_container.require::<Graphic2dService>();

        GizmosService {
            input_service,
            drag_service,
            graphic_service,
            graphic_2d_service,
        }
    }

    pub fn draw_square_helper(
        &self,
        corner1: Vector2D,
        corner2: Vector2D,
        color: Color,
        hover_color: Color,
        transform: Matrix3,
    ) -> bool {
        let bottom_left = Vector2D::new(
            f32::min(corner1.x, corner2.x),
            f32::min(corner1.y, corner2.y),
        );
        let top_right = Vector2D::new(
            f32::max(corner1.x, corner2.x),
            f32::max(corner1.y, corner2.y),
        );

        let is_hover = self.is_cursor_hover(&bottom_left, &top_right, &transform);
        let color = if is_hover { hover_color } else { color };

        let graphic_2d_service = self.graphic_2d_service.read();
        graphic_2d_service.draw_rect(
            bottom_left,
            top_right,
            3,
            Color::alpha(),
            color,
            1000,
            transform,
        );

        is_hover
    }

    pub fn draw_triangle_helper(
        &self,
        p1: Vector2D,
        p2: Vector2D,
        p3: Vector2D,
        color: Color,
        hover_color: Color,
        transform: Matrix3,
    ) -> bool {
        let cursor_pos = {
            let graphic_service = self.graphic_service.read();
            graphic_service.get_cursor_position()
        };

        let is_hover = {
            let p1 = transform * p1;
            let p2 = transform * p2;
            let p3 = transform * p3;
            cursor_pos.in_triangle(p1, p2, p3)
        };
        let color = if is_hover { hover_color } else { color };

        let graphic_2d_service = self.graphic_2d_service.read();
        graphic_2d_service.draw_line(p1, p2, 3, color, 1000, transform);
        graphic_2d_service.draw_line(p2, p3, 3, color, 1000, transform);
        graphic_2d_service.draw_line(p3, p1, 3, color, 1000, transform);

        is_hover
    }

    pub fn draw_circle_helper(
        &self,
        center: Vector2D,
        radius: f32,
        color: Color,
        hover_color: Color,
        transform: Matrix3,
    ) -> bool {
        let cursor_pos = {
            let graphic_service = self.graphic_service.read();
            graphic_service.get_cursor_position()
        };

        let is_hover = cursor_pos.in_circle(center, radius);
        let color = if is_hover { hover_color } else { color };

        let graphic_2d_service = self.graphic_2d_service.read();
        graphic_2d_service.draw_circle(center, radius, 3, color, Color::alpha(), 1000, transform);

        is_hover
    }

    pub fn draw_arrow_helper(
        &self,
        from: Vector2D,
        to: Vector2D,
        color: Color,
        hover_color: Color,
        transform: Matrix3,
    ) -> bool {
        let transform = Matrix3::new_translation(transform.translation());
        let graphic_2d_service = self.graphic_2d_service.read();
        let normalize = (to - from).normalize();
        let normal = normalize.normal();

        // Get camera transform
        let camera_transform = {
            let graphic_service = self.graphic_service.read();
            let transform: Matrix4 = transform.into();
            graphic_service.get_camera_transform() * transform
        };
        let camera_invert = camera_transform.invert();

        let is_hover = self.draw_triangle_helper(
            to - camera_invert * (normal * 0.025) - camera_invert * (normalize * 0.05),
            to + camera_invert * (normal * 0.025) - camera_invert * (normalize * 0.05),
            to,
            color,
            hover_color,
            transform,
        );

        let color = if is_hover { hover_color } else { color };
        graphic_2d_service.draw_line(
            from,
            to - camera_invert * (normalize * 0.05),
            3,
            color,
            1000,
            transform,
        );

        is_hover
    }

    pub fn draw_move_helper<FMove>(
        &self,
        center: Vector2D,
        size: Vector2D,
        color: Color,
        hover_color: Color,
        transform: Matrix3,
        on_move: FMove,
    ) -> FruityResult<()>
    where
        FMove: Fn(bool, bool) -> FruityResult<(DragCallback, DragEndCallback)>,
    {
        // Get camera transform
        let camera_transform = {
            let graphic_service = self.graphic_service.read();
            let transform: Matrix4 = transform.into();
            graphic_service.get_camera_transform() * transform
        };
        let camera_invert = camera_transform.invert();

        let move_handle_size = camera_invert * Vector2D::new(0.05, 0.05);
        let move_handle_size = Vector2D::new(
            f32::min(move_handle_size.x, move_handle_size.y),
            f32::min(move_handle_size.x, move_handle_size.y),
        );

        let top_right = Vector2D::new(center.x + size.x / 2.0, center.y + size.y / 2.0);

        // Draw free move helper
        let is_hover_free_move = self.draw_square_helper(
            center,
            center + move_handle_size,
            color,
            hover_color,
            transform,
        );

        // Draw the X arrow
        let from = (center + Vector2D::new(top_right.x, center.y)) / 2.0;
        let to =
            Vector2D::new(top_right.x, center.y) + Vector2D::new(move_handle_size.x * 2.0, 0.0);
        let is_hover_x_arrow = self.draw_arrow_helper(from, to, color, hover_color, transform);

        // Draw the Y arrow
        let from = (center + Vector2D::new(center.x, top_right.y)) / 2.0;
        let to =
            Vector2D::new(center.x, top_right.y) + Vector2D::new(0.0, move_handle_size.y * 2.0);
        let is_hover_y_arrow = self.draw_arrow_helper(from, to, color, hover_color, transform);

        // Implement the logic
        let input_service = self.input_service.read();

        // Handle moving
        if is_hover_free_move
            && input_service.is_source_pressed_this_frame("Mouse/Left".to_string())
        {
            let drag_service_reader = self.drag_service.read();
            drag_service_reader.start_drag(|| on_move(true, true))?;
        }

        if is_hover_x_arrow && input_service.is_source_pressed_this_frame("Mouse/Left".to_string())
        {
            let drag_service_reader = self.drag_service.read();
            drag_service_reader.start_drag(|| on_move(true, false))?;
        }

        if is_hover_y_arrow && input_service.is_source_pressed_this_frame("Mouse/Left".to_string())
        {
            let drag_service_reader = self.drag_service.read();
            drag_service_reader.start_drag(|| on_move(false, true))?;
        }

        // Handle moving
        if is_hover_free_move
            && input_service.is_source_pressed_this_frame("Mouse/Left".to_string())
        {
            let drag_service_reader = self.drag_service.read();
            drag_service_reader.start_drag(|| on_move(true, true))?;
        }

        if is_hover_x_arrow && input_service.is_source_pressed_this_frame("Mouse/Left".to_string())
        {
            let drag_service_reader = self.drag_service.read();
            drag_service_reader.start_drag(|| on_move(true, false))?;
        }

        if is_hover_y_arrow && input_service.is_source_pressed_this_frame("Mouse/Left".to_string())
        {
            let drag_service_reader = self.drag_service.read();
            drag_service_reader.start_drag(|| on_move(false, true))?;
        }

        Ok(())
    }

    pub fn draw_resize_helper<FResize>(
        &self,
        corner1: Vector2D,
        corner2: Vector2D,
        color: Color,
        hover_color: Color,
        transform: Matrix3,
        on_resize: FResize,
    ) -> FruityResult<()>
    where
        FResize: Fn(bool, bool) -> FruityResult<(DragCallback, DragEndCallback)>,
    {
        let transform_4: Matrix4 = transform.into();

        // Get camera and scale transform
        let bottom_left = Vector2D::new(
            f32::min(corner1.x, corner2.x),
            f32::min(corner1.y, corner2.y),
        );
        let top_right = Vector2D::new(
            f32::max(corner1.x, corner2.x),
            f32::max(corner1.y, corner2.y),
        );
        let bottom_right = Vector2D::new(top_right.x, bottom_left.y);
        let top_left = Vector2D::new(bottom_left.x, top_right.y);

        let resize_handle_size = {
            let camera_transform = {
                let graphic_service = self.graphic_service.read();
                graphic_service.get_camera_transform()
            };

            transform_4.invert() * camera_transform.invert() * Vector2D::new(0.01, 0.01)
        };

        // Draw the boundings
        self.draw_square_helper(bottom_left, top_right, color, hover_color, transform);

        // Draw bottom left
        let is_hover_resize_bottom_left = self.draw_square_helper(
            bottom_left,
            bottom_left + resize_handle_size,
            color,
            hover_color,
            transform,
        );

        // Draw bottom right
        let is_hover_resize_bottom_right = self.draw_square_helper(
            bottom_right,
            bottom_right + Vector2D::new(-resize_handle_size.x, resize_handle_size.y),
            color,
            hover_color,
            transform,
        );

        // Draw top left
        let is_hover_resize_top_left = self.draw_square_helper(
            top_left,
            top_left + Vector2D::new(resize_handle_size.x, -resize_handle_size.y),
            color,
            hover_color,
            transform,
        );

        // Draw top right
        let is_hover_resize_top_right = self.draw_square_helper(
            top_right,
            top_right - resize_handle_size,
            color,
            hover_color,
            transform,
        );

        // Implement the logic
        let input_service = self.input_service.read();

        // Handle resize
        if is_hover_resize_top_right
            && input_service.is_source_pressed_this_frame("Mouse/Left".to_string())
        {
            let drag_service_reader = self.drag_service.read();
            drag_service_reader.start_drag(|| on_resize(true, true))?;
        }

        if is_hover_resize_top_left
            && input_service.is_source_pressed_this_frame("Mouse/Left".to_string())
        {
            let drag_service_reader = self.drag_service.read();
            drag_service_reader.start_drag(|| on_resize(false, true))?;
        }

        if is_hover_resize_bottom_right
            && input_service.is_source_pressed_this_frame("Mouse/Left".to_string())
        {
            let drag_service_reader = self.drag_service.read();
            drag_service_reader.start_drag(|| on_resize(true, false))?;
        }

        if is_hover_resize_bottom_left
            && input_service.is_source_pressed_this_frame("Mouse/Left".to_string())
        {
            let drag_service_reader = self.drag_service.read();
            drag_service_reader.start_drag(|| on_resize(false, false))?;
        }

        Ok(())
    }

    fn is_cursor_hover(
        &self,
        bottom_left: &Vector2D,
        top_right: &Vector2D,
        transform: &Matrix3,
    ) -> bool {
        let graphic_service = self.graphic_service.read();
        let transform: Matrix4 = transform.clone().into();
        let bottom_left = transform * bottom_left.clone();
        let top_right = transform * top_right.clone();

        let cursor_pos = graphic_service.get_cursor_position();

        // Check if the cursor is in the rect
        bottom_left.x <= cursor_pos.x
            && cursor_pos.x <= top_right.x
            && bottom_left.y <= cursor_pos.y
            && cursor_pos.y <= top_right.y
    }
}
