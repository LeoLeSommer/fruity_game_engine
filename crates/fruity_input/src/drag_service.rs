use crate::InputService;
use fruity_game_engine::any::FruityAny;
use fruity_game_engine::export_impl;
use fruity_game_engine::export_struct;
use fruity_game_engine::profile_scope;
use fruity_game_engine::resource::ResourceContainer;
use fruity_game_engine::resource::ResourceReference;
use fruity_game_engine::signal::ObserverHandler;
use fruity_game_engine::sync::RwLock;
use fruity_game_engine::FruityResult;
use fruity_windows::window_service::WindowService;
use std::fmt::Debug;
use std::ops::Deref;
use std::ops::DerefMut;

pub type DragCallback = Box<dyn Fn(&DragAction) -> FruityResult<()> + Send + Sync + 'static>;
pub type DragEndCallback = Box<dyn Fn(&DragAction) -> FruityResult<()> + Send + Sync + 'static>;

pub struct DragAction {
    pub start_pos: (u32, u32),
    pub cursor_pos: (u32, u32),
    callback: DragCallback,
    end_callback: DragEndCallback,
}

impl Debug for DragAction {
    fn fmt(&self, _: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}

#[derive(Debug, FruityAny)]
#[export_struct]
pub struct DragService {
    current_drag_action: RwLock<Option<DragAction>>,
    input_service: ResourceReference<InputService>,
    window_service: ResourceReference<dyn WindowService>,
    on_end_update_handle: ObserverHandler<()>,
}

impl Drop for DragService {
    fn drop(&mut self) {
        self.on_end_update_handle.dispose_by_ref();
    }
}

#[export_impl]
impl DragService {
    pub fn new(resource_container: ResourceContainer) -> Self {
        let input_service = resource_container.require::<InputService>();
        let window_service = resource_container.require::<dyn WindowService>();
        let window_service_reader = window_service.read();

        let resource_container_2 = resource_container.clone();
        let on_end_update_handle = window_service_reader
            .on_end_update()
            .add_observer(move |_| {
                profile_scope!("update_drag");

                let drag_service = resource_container_2.require::<DragService>();
                let drag_service_reader = drag_service.read();

                drag_service_reader.update_drag()?;

                Ok(())
            });

        Self {
            current_drag_action: RwLock::new(None),
            input_service,
            window_service,
            on_end_update_handle: on_end_update_handle,
        }
    }

    pub fn start_drag(
        &self,
        start_callback: impl Fn() -> FruityResult<(DragCallback, DragEndCallback)>,
    ) -> FruityResult<()> {
        let start_pos = {
            let window_service_reader = self.window_service.read();
            window_service_reader.get_cursor_position()
        };

        let start_callback_result = start_callback()?;

        let drag_action = DragAction {
            start_pos,
            cursor_pos: start_pos,
            callback: start_callback_result.0,
            end_callback: start_callback_result.1,
        };

        let mut current_drag_action_writer = self.current_drag_action.write();
        *current_drag_action_writer = Some(drag_action);

        Ok(())
    }

    pub fn update_drag(&self) -> FruityResult<()> {
        // If the left mouse button is released, we stop dragging
        if !self.is_dragging_button_pressed() && self.is_dragging() {
            // Call the end action
            let mut current_drag_action_writer = self.current_drag_action.write();
            if let Some(current_drag_action) = current_drag_action_writer.deref_mut() {
                // Update cursor pos
                current_drag_action.cursor_pos = {
                    let window_service_reader = self.window_service.read();
                    window_service_reader.get_cursor_position()
                };

                (current_drag_action.end_callback)(&current_drag_action)?;

                // Clear the current action
                *current_drag_action_writer = None;
            }
        };

        // If a drag is active, we execute the associated callback
        let mut current_drag_action_writer = self.current_drag_action.write();
        if let Some(current_drag_action) = current_drag_action_writer.deref_mut() {
            // Update cursor pos
            current_drag_action.cursor_pos = {
                let window_service_reader = self.window_service.read();
                window_service_reader.get_cursor_position()
            };

            (current_drag_action.callback)(&current_drag_action)?;
        }

        Ok(())
    }

    fn is_dragging_button_pressed(&self) -> bool {
        let input_service = self.input_service.read();
        input_service.is_source_pressed("Mouse/Left".to_string())
    }

    fn is_dragging(&self) -> bool {
        let current_drag_action_reader = self.current_drag_action.read();

        if let Some(_) = current_drag_action_reader.deref() {
            true
        } else {
            false
        }
    }
}
