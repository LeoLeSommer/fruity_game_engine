use std::{ffi::c_void, rc::Rc};

use fruity_game_engine::{
    profile::{profile_new_frame, profile_scope, profile_start},
    settings::Settings,
    world::World,
    FruityResult,
};
use fruity_windows::window_service::WindowService;
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
    window::WindowBuilder,
};

use crate::{fps_counter::FPSCounter, window_service::WinitWindowService};

pub fn window_middleware(
    world: &World,
    settings: &Settings,
    load_resources: Rc<dyn Fn(&World) -> FruityResult<()>>,
    start: Rc<dyn Fn(&World) -> FruityResult<()>>,
    frame: Rc<dyn Fn(&World) -> FruityResult<()>>,
    end: Rc<dyn Fn(&World) -> FruityResult<()>>,
) -> FruityResult<()> {
    let resource_container = world.get_resource_container();

    // Read settings
    let window_settings = read_window_settings(settings);

    // Get windows base title
    let windows_title = window_settings.title.clone();

    // Build the window
    let event_loop = EventLoopBuilder::<()>::with_user_event().build();
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

    // Initialize windows service
    let window_service = WinitWindowService::new(resource_container.clone(), window);
    resource_container.add::<dyn WindowService>("window_service", Box::new(window_service));

    // Initialize the resources
    load_resources(world)?;

    // Build and inject the window in the windows service
    let (on_start_update, on_end_update, on_resize, on_cursor_moved, on_event, on_events_cleared) = {
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

    // Run the begin systems before everything
    start(world)?;

    // Run the render loop
    let window_service = resource_container.require::<dyn WindowService>();
    let window_service_reader = window_service.read();
    window_service_reader.on_enter_loop().notify(())?;
    std::mem::drop(window_service_reader);

    let mut fps_counter = FPSCounter::new();
    profile_start();

    let world = world.clone();
    event_loop.run(move |event, _, control_flow| {
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

        match event {
            // Check if the user has closed the window from the OS
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id: event_window_id,
                ..
            } => {
                if event_window_id == window_id {
                    // Run the end systems a the end
                    end(&world).unwrap();

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

        // Start updating
        {
            profile_scope("start_update");
            on_start_update.notify(()).unwrap();
        }

        // Run the systems
        {
            profile_scope("run_systems");

            frame(&world).unwrap();
        }

        // End the update
        {
            profile_scope("end_update");
            on_end_update.notify(()).unwrap();
        }

        // Update title with FPS
        {
            let fps = fps_counter.tick();
            let window_service = window_service.read();
            window_service.set_title(&format!("{} ({} FPS)", windows_title, fps));
        }
    });
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