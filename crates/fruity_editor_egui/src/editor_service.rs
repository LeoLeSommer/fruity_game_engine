use crate::ui_element::app::Application;
use crate::ui_element::app::DrawContext;
use egui::FontDefinitions;
use egui_wgpu_backend::RenderPass;
use egui_wgpu_backend::ScreenDescriptor;
use egui_winit_platform::Platform;
use egui_winit_platform::PlatformDescriptor;
use fruity_editor::ui::context::UIContext;
use fruity_game_engine::any::FruityAny;
use fruity_game_engine::export_impl;
use fruity_game_engine::export_struct;
use fruity_game_engine::frame_service::FrameService;
use fruity_game_engine::resource::resource_container::ResourceContainer;
use fruity_game_engine::resource::resource_reference::ResourceReference;
use fruity_game_engine::FruityError;
use fruity_game_engine::FruityResult;
use fruity_graphic::graphic_service::GraphicService;
use fruity_graphic_wgpu::graphic_service::WgpuGraphicService;
use fruity_windows::window_service::WindowService;
use fruity_windows_winit::window_service::WinitWindowService;
use std::fmt::Debug;
use winit::event::Event;

pub struct EditorServiceState {
    platform: Platform,
    egui_rpass: RenderPass,
    application: Application,
}

#[derive(FruityAny)]
#[export_struct]
pub struct EditorService {
    window_service: ResourceReference<dyn WindowService>,
    graphic_service: ResourceReference<dyn GraphicService>,
    frame_service: ResourceReference<FrameService>,
    state: EditorServiceState,
    ctx: UIContext,
}

impl Debug for EditorService {
    fn fmt(
        &self,
        _formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}

#[export_impl]
impl EditorService {
    pub fn new(resource_container: ResourceContainer) -> EditorService {
        let window_service = resource_container.require::<dyn WindowService>();
        let graphic_service = resource_container.require::<dyn GraphicService>();
        let frame_service = resource_container.require::<FrameService>();

        let window_service_reader = window_service.read();
        let window_service_reader = window_service_reader.downcast_ref::<WinitWindowService>();

        // Register to events of windows_service to update the event inputs when needed
        let resource_container_2 = resource_container.clone();
        window_service_reader.on_event().add_observer(move |event| {
            let editor_service = resource_container_2.require::<EditorService>();
            let mut editor_service = editor_service.write();

            editor_service.handle_event(&event);
            Ok(())
        });

        // Create the base UI
        let application = Application::new(resource_container.clone());

        // Connect to the window
        let state =
            EditorService::initialize(application, window_service.clone(), graphic_service.clone());

        EditorService {
            window_service: window_service.clone(),
            graphic_service: graphic_service.clone(),
            frame_service,
            state,
            ctx: UIContext::new(resource_container.clone()),
        }
    }

    pub fn initialize(
        application: Application,
        window_service: ResourceReference<dyn WindowService>,
        graphic_service: ResourceReference<dyn GraphicService>,
    ) -> EditorServiceState {
        let window_service = window_service.read();
        let graphic_service = graphic_service.read();
        let graphic_service = graphic_service.downcast_ref::<WgpuGraphicService>();

        // Get all what we need to initialize
        let device = graphic_service.get_device();
        let config = graphic_service.get_config();

        // We use the egui_winit_platform crate as the platform.
        let size = window_service.get_windows_size();
        let platform = Platform::new(PlatformDescriptor {
            physical_width: size.0 as u32,
            physical_height: size.1 as u32,
            scale_factor: window_service.get_scale_factor(),
            font_definitions: FontDefinitions::default(),
            style: Default::default(),
        });

        // We use the egui_wgpu_backend crate as the render backend.
        let egui_rpass = RenderPass::new(&device, config.format, 1);

        EditorServiceState {
            platform,
            egui_rpass,
            application,
        }
    }

    pub fn draw(&mut self) -> FruityResult<()> {
        let window_service = self.window_service.read();
        let window_service = window_service.downcast_ref::<WinitWindowService>();
        let graphic_service = self.graphic_service.read();
        let graphic_service = graphic_service.downcast_ref::<WgpuGraphicService>();

        let device = graphic_service.get_device();
        let config = graphic_service.get_config();
        let queue = graphic_service.get_queue();
        let rendering_view = graphic_service.get_rendering_view();
        let encoder = graphic_service.get_encoder().unwrap();

        let elapsed = {
            let frame_service_reader = self.frame_service.read();
            frame_service_reader.get_elapsed()
        };

        self.state.platform.update_time(elapsed);

        // Begin to draw the UI frame.
        self.state.platform.begin_frame();

        // Draw the application
        self.state.application.draw(
            &mut self.ctx,
            &mut DrawContext {
                device: device,
                platform: &self.state.platform,
                egui_rpass: &mut self.state.egui_rpass,
            },
        )?;

        // End the UI frame. We could now handle the output and draw the UI with the backend.
        let full_output = self
            .state
            .platform
            .end_frame(Some(window_service.get_window()));

        let paint_jobs = self.state.platform.context().tessellate(full_output.shapes);

        // Upload all resources for the GPU.
        let screen_descriptor = ScreenDescriptor {
            physical_width: config.width,
            physical_height: config.height,
            scale_factor: window_service.get_scale_factor() as f32,
        };
        let tdelta: egui::TexturesDelta = full_output.textures_delta;
        self.state
            .egui_rpass
            .add_textures(&device, &queue, &tdelta)
            .map_err(|err| FruityError::GenericFailure(err.to_string()))?;
        self.state
            .egui_rpass
            .update_buffers(&device, &queue, &paint_jobs, &screen_descriptor);

        // Record all render passes.
        let mut encoder = encoder.write();
        self.state
            .egui_rpass
            .execute(
                &mut encoder,
                &rendering_view,
                &paint_jobs,
                &screen_descriptor,
                None,
            )
            .unwrap();

        Ok(())
    }

    fn handle_event(&mut self, event: &Event<'static, ()>) {
        self.state.platform.handle_event(&event);
    }

    pub fn get_egui_rpass(&mut self) -> &mut RenderPass {
        &mut self.state.egui_rpass
    }
}
