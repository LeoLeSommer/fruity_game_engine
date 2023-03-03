use fruity_game_engine::{
    console_log,
    frame_service::FrameService,
    profile::{profile_new_frame, profile_scope, profile_start},
    settings::Settings,
    world::World,
    FruityError, FruityResult,
};
use fruity_windows::window_service::WindowService;
use std::{ffi::c_void, future::Future, pin::Pin};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
    window::WindowBuilder,
};

use crate::{fps_counter::FPSCounter, window_service::WinitWindowService};

pub fn window_middleware(
    world: World,
    settings: Settings,
) -> Pin<Box<dyn Future<Output = FruityResult<()>>>> {
    Box::pin(async move {
        let resource_container = world.get_resource_container();
        console_log("1");

        // Read settings
        let window_settings = read_window_settings(&settings);

        // Get windows base title
        let windows_title = window_settings.title.clone();

        // Build the window
        let event_loop = EventLoopBuilder::<()>::with_user_event().build();
        console_log("2");

        let window = WindowBuilder::new()
            .with_title(window_settings.title)
            .with_inner_size(LogicalSize::new(
                window_settings.width as u32,
                window_settings.height as u32,
            ))
            .with_resizable(window_settings.resizable)
            .build(&event_loop)
            .unwrap();

        let window_id = window.id();
        console_log("3");

        // On wasm, append the canvas to the document body
        #[cfg(target_arch = "wasm32")]
        {
            use winit::platform::web::WindowExtWebSys;
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| doc.body())
                .and_then(|body| {
                    body.append_child(&web_sys::Element::from(window.canvas()))
                        .ok()
                })
                .ok_or(FruityError::GenericFailure(
                    "couldn't append canvas to document body".to_string(),
                ))?;
        }

        // Initialize windows service
        let window_service = WinitWindowService::new(resource_container.clone(), window);
        resource_container.add::<dyn WindowService>("window_service", Box::new(window_service));
        console_log("4");

        // Setup modules
        world.setup_modules().await?;
        console_log("5");

        // Initialize the resources
        world.load_resources()?;
        console_log("6");

        // Get the windows events
        let (
            on_start_update,
            on_end_update,
            on_resize,
            on_cursor_moved,
            on_event,
            on_events_cleared,
        ) = {
            let window_service = resource_container.require::<dyn WindowService>();
            let window_service_reader = window_service.read();
            let window_service_reader = window_service_reader.downcast_ref::<WinitWindowService>();

            (
                window_service_reader.on_start_update.clone(),
                window_service_reader.on_end_update.clone(),
                window_service_reader.on_resize.clone(),
                window_service_reader.on_cursor_moved.clone(),
                window_service_reader.on_event().clone(),
                window_service_reader.on_events_cleared.clone(),
            )
        };
        console_log("7");

        // Run the begin systems before everything
        world.start()?;
        console_log("8");

        // Run the render loop
        let frame_service = resource_container.require::<FrameService>();
        let window_service = resource_container.require::<dyn WindowService>();
        let window_service_reader = window_service.read();
        window_service_reader.on_enter_loop().notify(())?;
        std::mem::drop(window_service_reader);
        console_log("9");

        let mut fps_counter = FPSCounter::new();
        profile_start();
        console_log("10");

        event_loop.run(move |event, _, control_flow| {
            console_log("11");
            // Update FrameService current tick
            {
                let mut frame_service = frame_service.write();
                frame_service.begin_frame();
            }
            console_log("12");

            profile_new_frame();
            profile_scope("main_loop");
            *control_flow = ControlFlow::Wait;

            // Handle events
            {
                profile_scope("handle events");

                // TODO: Try to find a way to remove this
                let event = &event as *const _ as *const c_void;
                let event = event as *const Event<'static, ()>;
                let event = unsafe { &*event as &Event<'static, ()> };
                let event = unsafe { &*(&event as *const _) } as &Event<'static, ()>;
                let event = event.clone();
                on_event.notify(event).unwrap();
            }
            console_log("13");

            match event {
                // Check if the user has closed the window from the OS
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id: event_window_id,
                    ..
                } => {
                    if event_window_id == window_id {
                        // Run the end systems a the end
                        world.end().unwrap();

                        // Transmit to the loop that it should end
                        *control_flow = ControlFlow::Exit;
                    }
                }
                // Check if the user has resized the window from the OS
                Event::WindowEvent {
                    event: WindowEvent::Resized(physical_size),
                    ..
                } => {
                    on_resize
                        .notify((physical_size.width, physical_size.height))
                        .unwrap();
                }
                // Check if the user has moved the cursor
                Event::WindowEvent {
                    event: WindowEvent::CursorMoved { position, .. },
                    ..
                } => {
                    let mut window_service = window_service.write();
                    let mut window_service = window_service
                        .as_any_mut()
                        .downcast_mut::<WinitWindowService>()
                        .unwrap();

                    window_service.cursor_position = (position.x as u32, position.y as u32);
                    std::mem::drop(window_service);

                    on_cursor_moved
                        .notify((position.x as u32, position.y as u32))
                        .unwrap();
                }
                Event::WindowEvent {
                    event: WindowEvent::ScaleFactorChanged { new_inner_size, .. },
                    ..
                } => {
                    on_resize
                        .notify((new_inner_size.width, new_inner_size.height))
                        .unwrap();
                }
                Event::MainEventsCleared => {
                    on_events_cleared.notify(()).unwrap();
                }
                _ => (),
            }
            console_log("14");

            // Start updating
            {
                profile_scope("start_update");
                on_start_update.notify(()).unwrap();
            }
            console_log("15");

            // Run the systems
            {
                profile_scope("run_systems");
                world.frame().unwrap();
            }
            console_log("16");

            // End the update
            {
                profile_scope("end_update");
                on_end_update.notify(()).unwrap();
            }
            console_log("17");

            // Update title with FPS
            {
                let fps = fps_counter.tick();
                let window_service = window_service.read();
                window_service.set_title(&format!("{} ({} FPS)", windows_title, fps));
            }
            console_log("18");
        });
    })
}

struct WindowSettings {
    title: String,
    width: usize,
    height: usize,
    resizable: bool,
}

fn read_window_settings(settings: &Settings) -> WindowSettings {
    let settings = settings.get_settings("window");

    WindowSettings {
        title: settings.get("title", "".to_string()),
        width: settings.get("width", 512),
        height: settings.get("height", 512),
        resizable: settings.get("resizable", true),
    }
}
