use crate::ui_element::DrawContext;
use fruity_ecs::system::SystemService;
use fruity_editor::ui::context::UIContext;
use fruity_editor::ui::hooks::use_memo;
use fruity_editor::ui::hooks::use_read_service;
use fruity_editor::ui::hooks::use_state;
use fruity_game_engine::resource::resource_reference::ResourceReference;
use fruity_game_engine::FruityResult;
use fruity_game_engine::RwLock;
use fruity_graphic::graphic_service::GraphicService;
use fruity_graphic::math::matrix4::Matrix4;
use fruity_graphic::math::vector2d::Vector2D;
use fruity_graphic::math::Color;
use fruity_graphic::resources::texture_resource::TextureResource;
use fruity_graphic_wgpu::graphic_service::WgpuGraphicService;
use fruity_graphic_wgpu::resources::texture_resource::WgpuTextureResource;
use std::sync::Arc;

pub fn draw_scene(
    ctx: &mut UIContext,
    ui: &mut egui::Ui,
    draw_ctx: &mut DrawContext,
) -> FruityResult<()> {
    // Initialize local state
    let (center, _set_center) = use_state(ctx, Vector2D::default());
    let (zoom, set_zoom) = use_state(ctx, 4.0 as f32);

    // Get available dimensions
    let rect = ui.available_rect_before_wrap();
    let width = (ui.available_width() / ui.input(|input| input.physical_pixel_size())) as u32;
    let height = (ui.available_height() / ui.input(|input| input.physical_pixel_size())) as u32;
    let ratio = ui.available_width() / ui.available_height();

    // Update viewport properties
    {
        let graphic_service = use_read_service::<dyn GraphicService>(ctx);
        graphic_service.set_viewport_offset(rect.left() as u32 * 2, rect.top() as u32 * 2);
        graphic_service.set_viewport_size(rect.width() as u32 * 2, rect.height() as u32 * 2);
    }

    // Update camera if needed
    let is_cursor_hover_scene = {
        let graphic_service = use_read_service::<dyn GraphicService>(ctx);
        graphic_service.is_cursor_hover_scene()
    };

    if is_cursor_hover_scene {
        if ui.input(|input| input.scroll_delta.y) != 0.0 {
            set_zoom(zoom + ui.input(|input| input.scroll_delta.y) * 0.001);
        }
    }

    // Calculate the camera view transform
    let zoom = f32::powf(2.0, zoom as f32);
    let view_proj = Matrix4::from_rect(
        center.x - zoom,
        center.x + zoom,
        center.y - (zoom / ratio),
        center.y + (zoom / ratio),
        -1.0,
        1.0,
    );

    // Build the rendering texture
    let (resource, rendering_texture_id) = use_memo(
        ctx,
        |ctx| {
            // Get all what we need to initialize
            let graphic_service = use_read_service::<dyn GraphicService>(&ctx);
            let graphic_service = graphic_service.downcast_ref::<WgpuGraphicService>();

            let device = graphic_service.get_device();
            let surface_config = graphic_service.get_config();

            // Create the rendering texture resource
            let resource = ResourceReference::new(
                "Rendering View",
                Arc::new(RwLock::new(Box::new(WgpuTextureResource::render(
                    device,
                    surface_config,
                    width,
                    height,
                    "Rendering View",
                )) as Box<dyn TextureResource>)),
            );

            // Use the texture as the rendering texture
            let image = resource.read();
            let image = image.downcast_ref::<WgpuTextureResource>();

            // Get the egui identifier for the texture
            (
                resource,
                draw_ctx.egui_rpass.egui_texture_from_wgpu_texture(
                    draw_ctx.device,
                    image.get_view(),
                    wgpu::FilterMode::Linear,
                ),
            )
        },
        (width, height),
    );

    // Draw the scene on the texture
    let background_color = ui.style().visuals.faint_bg_color;
    let background_color = Color::new(
        background_color.r() as f32 / 255.0,
        background_color.g() as f32 / 255.0,
        background_color.b() as f32 / 255.0,
        background_color.a() as f32 / 255.0,
    );

    // Run the systems
    {
        let system_service = use_read_service::<SystemService>(ctx);
        system_service.run_frame()?;
    }

    let graphic_service = use_read_service::<dyn GraphicService>(ctx);
    graphic_service.render_scene(view_proj, background_color, Some(resource.clone()));

    // Display the scene
    ui.add_sized(
        rect.size(),
        egui::Image::new(rendering_texture_id, rect.size()),
    );

    Ok(())
}
