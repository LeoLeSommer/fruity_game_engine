use adjacent_pair_iterator::AdjacentPairIterator;
use fruity_game_engine::any::FruityAny;
use fruity_game_engine::export;
use fruity_game_engine::export_impl;
use fruity_game_engine::export_struct;
use fruity_game_engine::resource::resource_container::ResourceContainer;
use fruity_game_engine::resource::resource_reference::ResourceReference;
use fruity_game_engine::resource::Resource;
use fruity_game_engine::FruityError;
use fruity_game_engine::FruityResult;
use fruity_graphic::graphic_service::GraphicService;
use fruity_graphic::graphic_service::MaterialParam;
use fruity_graphic::math::matrix3::Matrix3;
use fruity_graphic::math::vector2d::Vector2D;
use fruity_graphic::math::Color;
use fruity_graphic::resources::material_resource::MaterialResource;
use fruity_graphic::resources::mesh_resource::MeshResource;
use maplit::hashmap;
use std::collections::HashMap;
use std::f32::consts::PI;
use std::ops::Range;

#[derive(Debug, FruityAny, Resource)]
#[export_struct]
pub struct Graphic2dService {
    graphic_service: ResourceReference<dyn GraphicService>,
    resource_container: ResourceContainer,
    draw_line_material: ResourceReference<dyn MaterialResource>,
    draw_dotted_line_material: ResourceReference<dyn MaterialResource>,
    draw_rect_material: ResourceReference<dyn MaterialResource>,
    draw_arc_material: ResourceReference<dyn MaterialResource>,
}

#[export_impl]
impl Graphic2dService {
    pub fn new(resource_container: ResourceContainer) -> FruityResult<Self> {
        let graphic_service = resource_container.require::<dyn GraphicService>();

        let draw_line_material = resource_container
            .get::<dyn MaterialResource>("Materials/Draw Line")
            .ok_or(FruityError::GenericFailure(format!(
                "Missing shader {}",
                "Materials/Draw Line"
            )))?;

        let draw_dotted_line_material = resource_container
            .get::<dyn MaterialResource>("Materials/Draw Dotted Line")
            .ok_or(FruityError::GenericFailure(format!(
                "Missing shader {}",
                "Materials/Draw Dotted Line"
            )))?;

        let draw_rect_material = resource_container
            .get::<dyn MaterialResource>("Materials/Draw Rect")
            .ok_or(FruityError::GenericFailure(format!(
                "Missing shader {}",
                "Materials/Draw Rect"
            )))?;

        let draw_arc_material = resource_container
            .get::<dyn MaterialResource>("Materials/Draw Arc")
            .ok_or(FruityError::GenericFailure(format!(
                "Missing shader {}",
                "Materials/Draw Arc"
            )))?;

        Ok(Self {
            graphic_service,
            resource_container,
            draw_line_material,
            draw_dotted_line_material,
            draw_rect_material,
            draw_arc_material,
        })
    }

    #[export]
    pub fn draw_quad(
        &self,
        identifier: u64,
        material: ResourceReference<dyn MaterialResource>,
        params: HashMap<String, MaterialParam>,
        z_index: i32,
    ) {
        let graphic_service = self.graphic_service.read();

        let mesh = self
            .resource_container
            .get::<dyn MeshResource>("Meshes/Squad")
            .unwrap();

        graphic_service.draw_mesh(identifier, mesh.clone(), material, params, z_index)
    }

    #[export]
    pub fn draw_line(
        &self,
        pos1: Vector2D,
        pos2: Vector2D,
        width: u32,
        color: Color,
        z_index: i32,
        transform: Matrix3,
    ) {
        self.draw_quad(
            0,
            self.draw_line_material.clone(),
            hashmap! {
                "transform".to_string() => MaterialParam::Matrix4(transform.into()),
                "pos1".to_string() => MaterialParam::Vector2D(pos1),
                "pos2".to_string() => MaterialParam::Vector2D(pos2),
                "width".to_string() => MaterialParam::Uint(width),
                "color".to_string() => MaterialParam::Color(color),
            },
            z_index,
        );
    }

    #[export]
    pub fn draw_polyline(
        &self,
        points: Vec<Vector2D>,
        width: u32,
        color: Color,
        z_index: i32,
        transform: Matrix3,
    ) {
        points
            .adjacent_pairs()
            .for_each(|(pos1, pos2)| self.draw_line(pos1, pos2, width, color, z_index, transform));
    }

    #[export]
    pub fn draw_dotted_line(
        &self,
        pos1: Vector2D,
        pos2: Vector2D,
        width: u32,
        color: Color,
        z_index: i32,
        transform: Matrix3,
    ) {
        self.draw_quad(
            0,
            self.draw_dotted_line_material.clone(),
            hashmap! {
                "transform".to_string() => MaterialParam::Matrix4(transform.into()),
                "pos1".to_string() => MaterialParam::Vector2D(pos1),
                "pos2".to_string() => MaterialParam::Vector2D(pos2),
                "width".to_string() => MaterialParam::Uint(width),
                "color".to_string() => MaterialParam::Color(color),
            },
            z_index,
        );
    }

    #[export]
    pub fn draw_rect(
        &self,
        bottom_left: Vector2D,
        top_right: Vector2D,
        width: u32,
        fill_color: Color,
        border_color: Color,
        z_index: i32,
        transform: Matrix3,
    ) {
        self.draw_quad(
            0,
            self.draw_rect_material.clone(),
            hashmap! {
                "transform".to_string() => MaterialParam::Matrix4(transform.into()),
                "bottom_left".to_string() => MaterialParam::Vector2D(bottom_left),
                "top_right".to_string() => MaterialParam::Vector2D(top_right),
                "width".to_string() => MaterialParam::Uint(width),
                "fill_color".to_string() => MaterialParam::Color(fill_color),
                "border_color".to_string() => MaterialParam::Color(border_color),
            },
            z_index,
        );
    }

    #[export]
    pub fn draw_arc(
        &self,
        center: Vector2D,
        radius: f32,
        angle_range: Range<f32>,
        width: u32,
        fill_color: Color,
        border_color: Color,
        z_index: i32,
        transform: Matrix3,
    ) {
        // Calculate angle range
        let angle_range = normalize_angle_range(angle_range);

        // Draw the arc
        self.draw_quad(
            0,
            self.draw_arc_material.clone(),
            hashmap! {
                "transform".to_string() => MaterialParam::Matrix4(transform.into()),
                "center".to_string() => MaterialParam::Vector2D(center),
                "radius".to_string() => MaterialParam::Float(radius),
                "fill_color".to_string() => MaterialParam::Color(fill_color),
                "border_color".to_string() => MaterialParam::Color(border_color),
                "width".to_string() => MaterialParam::Uint(width),
                "angle_start".to_string() => MaterialParam::Float(angle_range.start),
                "angle_end".to_string() => MaterialParam::Float(angle_range.end),
            },
            z_index,
        );
    }

    #[export]
    pub fn draw_circle(
        &self,
        center: Vector2D,
        radius: f32,
        width: u32,
        fill_color: Color,
        border_color: Color,
        z_index: i32,
        transform: Matrix3,
    ) {
        self.draw_arc(
            center,
            radius,
            0.0..(2.0 * PI),
            width,
            fill_color,
            border_color,
            z_index,
            transform,
        );
    }
}

/// Take a radian angle and normalize it between [-PI, PI[
///
/// # Arguments
/// * `angle` - The input angle
///
pub fn normalize_angle(angle: f32) -> f32 {
    if angle < -PI {
        normalize_angle(angle + 2.0 * PI)
    } else if angle >= PI {
        normalize_angle(angle - 2.0 * PI)
    } else {
        angle
    }
}

/// Take a radian angle range and normalize each born between [-PI, PI[
/// If the range length is 2PI, returns simply -PI..PI
///
/// # Arguments
/// * `range` - The input range
///
pub fn normalize_angle_range(range: Range<f32>) -> Range<f32> {
    if range.start == range.end {
        return 0.0..0.0;
    }

    let angle1 = normalize_angle(range.start);
    let angle2 = normalize_angle(range.end);

    let start = f32::min(angle1, angle2);
    let end = f32::max(angle1, angle2);

    if start == end {
        -PI..PI
    } else {
        start..end
    }
}

#[cfg(test)]
mod tests {
    use crate::graphic_2d_service::normalize_angle;
    use crate::graphic_2d_service::normalize_angle_range;
    use std::f32::consts::PI;

    #[test]
    fn normalize_angle_test() {
        assert_eq!(normalize_angle(3.0 * PI / 4.0), 3.0 * PI / 4.0);
        assert_eq!(normalize_angle(6.0 * PI / 4.0), -2.0 * PI / 4.0);
        assert_eq!(normalize_angle(PI), -PI);
        assert_eq!(normalize_angle(-PI), -PI);
    }

    #[test]
    fn normalize_angle_range_test() {
        assert_eq!(
            normalize_angle_range((3.0 * PI / 4.0)..(6.0 * PI / 4.0)),
            (-2.0 * PI / 4.0)..(3.0 * PI / 4.0)
        );
        assert_eq!(
            normalize_angle_range(0.0..(6.0 * PI / 4.0)),
            0.0..(2.0 * PI / 4.0)
        );
        assert_eq!(normalize_angle_range(0.0..(2.0 * PI)), -PI..PI);
        assert_eq!(
            normalize_angle_range((3.0 * PI / 4.0)..(3.0 * PI / 4.0)),
            -PI..PI
        );
        assert_eq!(normalize_angle_range(0.0..0.0), 0.0..0.0);
    }
}
