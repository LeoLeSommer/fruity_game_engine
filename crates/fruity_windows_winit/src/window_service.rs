use fruity_game_engine::any::FruityAny;
use fruity_game_engine::resource::resource_container::ResourceContainer;
use fruity_game_engine::send_wrapper::SendWrapper;
use fruity_game_engine::signal::Signal;
use fruity_game_engine::{export, export_impl, export_struct, FruityResult};
use fruity_windows::window_service::WindowService;
use std::fmt::Debug;
use winit::dpi::LogicalSize;
use winit::event::Event;
use winit::event_loop::EventLoop;
use winit::window::Window;

#[cfg(target_arch = "wasm32")]
unsafe impl Sync for WinitWindowService {}

#[cfg(target_arch = "wasm32")]
unsafe impl Send for WinitWindowService {}

#[derive(FruityAny)]
#[export_struct]
pub struct WinitWindowService {
    window: Window,
    event_loop: Option<SendWrapper<EventLoop<()>>>,
    pub cursor_position: (u32, u32),
    pub on_start_update: Signal<()>,
    pub on_end_update: Signal<()>,
    pub on_resize: Signal<(u32, u32)>,
    pub on_cursor_moved: Signal<(u32, u32)>,
    pub on_events_cleared: Signal<()>,
    on_event: Signal<Event<'static, ()>>,
}

impl Debug for WinitWindowService {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}

#[export_impl]
impl WinitWindowService {
    pub fn new(
        _resource_container: ResourceContainer,
        window: Window,
        event_loop: EventLoop<()>,
    ) -> WinitWindowService {
        WinitWindowService {
            window,
            event_loop: Some(SendWrapper::new(event_loop)),
            cursor_position: Default::default(),
            on_start_update: Signal::new(),
            on_end_update: Signal::new(),
            on_resize: Signal::new(),
            on_cursor_moved: Signal::new(),
            on_event: Signal::new(),
            on_events_cleared: Signal::new(),
        }
    }

    pub fn get_window(&self) -> &Window {
        &self.window
    }

    pub fn take_event_loop(&mut self) -> EventLoop<()> {
        self.event_loop.take().unwrap().take()
    }

    pub fn on_event(&self) -> &Signal<Event<'static, ()>> {
        &self.on_event
    }
}

impl WindowService for WinitWindowService {
    #[export]
    fn close(&self) {
        // TODO: Repair that
        //window.se.push(WindowEvent::CloseRequested);
    }

    #[export]
    fn set_resizable(&self, resizable: bool) {
        self.get_window().set_resizable(resizable);
    }

    #[export]
    fn get_windows_size(&self) -> (u32, u32) {
        (
            self.get_window().inner_size().width,
            self.get_window().inner_size().height,
        )
    }

    #[export]
    fn get_scale_factor(&self) -> f64 {
        self.get_window().scale_factor()
    }

    #[export]
    fn get_cursor_position(&self) -> (u32, u32) {
        self.cursor_position.clone()
    }

    #[export]
    fn set_size(&self, width: u32, height: u32) -> FruityResult<()> {
        self.get_window()
            .set_inner_size(LogicalSize::new(width as i32, height as i32));

        self.on_resize.send((width, height))
    }

    #[export]
    fn set_title(&self, title: &str) {
        self.get_window().set_title(title);
    }

    fn on_start_update(&self) -> &Signal<()> {
        &self.on_start_update
    }

    fn on_end_update(&self) -> &Signal<()> {
        &self.on_end_update
    }

    fn on_resize(&self) -> &Signal<(u32, u32)> {
        &self.on_resize
    }

    fn on_cursor_moved(&self) -> &Signal<(u32, u32)> {
        &self.on_cursor_moved
    }
}
