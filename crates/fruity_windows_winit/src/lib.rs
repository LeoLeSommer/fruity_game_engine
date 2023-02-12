use crate::window_service::WinitWindowService;
use fruity_game_engine::export_function;
use fruity_game_engine::frame_service::FrameService;
use fruity_game_engine::module::Module;
use fruity_game_engine::profile::profile_scope;
use fruity_windows::window_service::WindowService;
use std::rc::Rc;

pub mod fps_counter;
pub mod window_middleware;
pub mod window_service;

/// Returns the module, ready to be registered into the fruity_game_engine
#[export_function]
pub fn create_fruity_windows_winit_module() -> Module {
    Module {
        name: "fruity_windows".to_string(),
        dependencies: vec!["fruity_abstract_windows".to_string()],
        setup: Some(Rc::new(|world, _settings| {
            let resource_container = world.get_resource_container();

            let window_service = resource_container.require::<dyn WindowService>();
            let window_service_reader = window_service.read();
            let window_service_reader = window_service_reader.downcast_ref::<WinitWindowService>();

            let resource_container_2 = resource_container.clone();
            window_service_reader.on_enter_loop.add_observer(move |_| {
                let frame_service = resource_container_2.require::<FrameService>();
                let mut frame_service = frame_service.write();

                frame_service.begin_frame();

                Ok(())
            });

            let resource_container_2 = resource_container.clone();
            window_service_reader
                .on_start_update
                .add_observer(move |_| {
                    profile_scope("begin_frame");

                    let frame_service = resource_container_2.require::<FrameService>();
                    let mut frame_service = frame_service.write();

                    frame_service.begin_frame();

                    Ok(())
                });

            Ok(())
        })),
        load_resources: None,
    }
}
