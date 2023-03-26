use crate::window_service::WinitWindowService;
use fruity_game_engine::{
    frame_service::FrameService,
    profile::{profile_new_frame, profile_start},
    profile_scope,
    settings::Settings,
    world::{RunWorldMiddlewareNext, SetupWorldMiddlewareNext, World},
    FruityResult,
};
use fruity_windows::window_service::WindowService;
use std::ffi::c_void;
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder, EventLoopWindowTarget},
    window::WindowBuilder,
};

#[cfg(target_arch = "wasm32")]
use winit::platform::web::EventLoopExtWebSys;

pub fn setup_world_middleware(
    world: World,
    settings: Settings,
    next: SetupWorldMiddlewareNext,
) -> FruityResult<()> {
    let resource_container = world.get_resource_container();

    // Read settings
    let window_settings = read_window_settings(&settings);

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
            .ok_or(fruity_game_engine::FruityError::GenericFailure(
                "couldn't append canvas to document body".to_string(),
            ))?;
    }

    // Initialize windows service
    let window_service = WinitWindowService::new(resource_container.clone(), window, event_loop);
    resource_container.add::<dyn WindowService>("window_service", Box::new(window_service));

    // Run next
    next(world.clone(), settings.clone())
}

pub fn run_world_middleware(
    world: World,
    settings: Settings,
    next: RunWorldMiddlewareNext,
) -> FruityResult<()> {
    // Run next
    next(world.clone(), settings.clone())?;

    let resource_container = world.get_resource_container();

    // Get the windows events
    let (
        event_loop,
        window_id,
        on_start_update,
        on_end_update,
        on_resize,
        on_cursor_moved,
        on_event,
        on_events_cleared,
    ) = {
        let window_service = resource_container.require::<dyn WindowService>();
        let mut window_service_writer = window_service.write();
        let window_service = window_service_writer.downcast_mut::<WinitWindowService>();

        (
            window_service.take_event_loop(),
            window_service.get_window().id(),
            window_service.on_start_update.clone(),
            window_service.on_end_update.clone(),
            window_service.on_resize.clone(),
            window_service.on_cursor_moved.clone(),
            window_service.on_event().clone(),
            window_service.on_events_cleared.clone(),
        )
    };

    // Run the begin systems before everything
    world.start()?;

    // Run the render loop
    let frame_service = resource_container.require::<FrameService>();
    let window_service = resource_container.require::<dyn WindowService>();

    profile_start();

    let loop_closure = move |event: Event<'_, ()>,
                             _: &EventLoopWindowTarget<()>,
                             control_flow: &mut ControlFlow| {
        profile_new_frame();
        profile_scope!("main_loop");

        // Update FrameService current tick
        {
            profile_scope!("frame_service_begin_frame");
            let mut frame_service = frame_service.write();
            frame_service.begin_frame();
        }

        *control_flow = ControlFlow::Wait;

        // Handle events
        {
            profile_scope!("handle events");

            // TODO: Try to find a way to remove this
            let event = &event as *const _ as *const c_void;
            let event = event as *const Event<'static, ()>;
            let event = unsafe { &*event as &Event<'static, ()> };
            let event = unsafe { &*(&event as *const _) } as &Event<'static, ()>;

            on_event.notify(event.clone()).unwrap();

            match event {
                // Check if the user has closed the window from the OS
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id: event_window_id,
                    ..
                } => {
                    if *event_window_id == window_id {
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
        }

        // Start updating
        {
            profile_scope!("start_update");
            on_start_update.notify(()).unwrap();
        }

        // Run the systems
        {
            world.frame().unwrap();
        }

        // End the update
        {
            profile_scope!("end_update");
            on_end_update.notify(()).unwrap();
        }
    };

    #[cfg(not(target_arch = "wasm32"))]
    event_loop.run(loop_closure);

    #[cfg(target_arch = "wasm32")]
    event_loop.spawn(loop_closure);

    #[cfg(target_arch = "wasm32")]
    Ok(())
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
