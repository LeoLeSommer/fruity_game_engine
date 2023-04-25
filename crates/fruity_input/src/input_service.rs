use fruity_game_engine::any::FruityAny;
use fruity_game_engine::export;
use fruity_game_engine::export_impl;
use fruity_game_engine::export_struct;
use fruity_game_engine::resource::ResourceContainer;
use fruity_game_engine::settings::Settings;
use fruity_game_engine::signal::Signal;
use fruity_game_engine::FruityResult;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;

pub const SHIFT: u32 = 0x01;
pub const CTRL: u32 = 0x02;
pub const ALT: u32 = 0x04;
pub const LOGO: u32 = 0x08;

#[derive(Default, Debug, Clone, FruityAny)]
#[export_struct]
pub struct Modifiers(pub u32);

#[export_impl]
impl Modifiers {
    /// Returns `true` if the shift key is pressed.
    #[export]
    pub fn has_shift(&self) -> bool {
        self.0 & (1 << (SHIFT - 1)) != 0
    }
    /// Returns `true` if the control key is pressed.
    #[export]
    pub fn has_ctrl(&self) -> bool {
        self.0 & (1 << (CTRL - 1)) != 0
    }
    /// Returns `true` if the alt key is pressed.
    #[export]
    pub fn has_alt(&self) -> bool {
        self.0 & (1 << (ALT - 1)) != 0
    }
    /// Returns `true` if the logo key is pressed.
    #[export]
    pub fn has_logo(&self) -> bool {
        self.0 & (1 << (LOGO - 1)) != 0
    }
}

#[derive(Debug, FruityAny)]
#[export_struct]
pub struct InputService {
    pub input_map: HashMap<String, String>,
    pub pressed_inputs: HashSet<String>,
    pub pressed_sources: HashSet<String>,
    pub pressed_modifiers: Modifiers,
    pub pressed_this_frame_inputs: HashSet<String>,
    pub pressed_this_frame_sources: HashSet<String>,
    pub released_this_frame_inputs: HashSet<String>,
    pub released_this_frame_sources: HashSet<String>,
    pub on_pressed: Signal<String>,
    pub on_released: Signal<String>,
}

#[export_impl]
impl InputService {
    pub fn new(_resource_container: ResourceContainer) -> InputService {
        InputService {
            input_map: HashMap::new(),
            pressed_inputs: HashSet::new(),
            pressed_sources: HashSet::new(),
            pressed_modifiers: Default::default(),
            pressed_this_frame_inputs: HashSet::new(),
            pressed_this_frame_sources: HashSet::new(),
            released_this_frame_inputs: HashSet::new(),
            released_this_frame_sources: HashSet::new(),
            on_pressed: Signal::new(),
            on_released: Signal::new(),
        }
    }

    pub fn read_input_settings(&mut self, settings: &Settings) -> FruityResult<()> {
        let input_settings = settings.get("input", Settings::default());

        if let Settings::Array(inputs_settings) = input_settings {
            inputs_settings.iter().try_for_each(|input_settings| {
                let name = input_settings.get::<String>("name", "".to_string());
                let sources = input_settings.get::<Vec<String>>("source", vec![]);

                if name.len() > 0 && sources.len() > 0 {
                    sources
                        .into_iter()
                        .for_each(|source| self.register_input(name.clone(), source));
                }

                FruityResult::Ok(())
            })?;
        }

        Ok(())
    }

    #[export]
    pub fn register_input(&mut self, input: String, source: String) {
        self.input_map.insert(source, input);
    }

    #[export]
    pub fn is_pressed(&self, input: String) -> bool {
        self.pressed_inputs.contains(&input)
    }

    #[export]
    pub fn is_source_pressed(&self, source: String) -> bool {
        self.pressed_sources.contains(&source)
    }

    #[export]
    pub fn is_pressed_this_frame(&self, input: String) -> bool {
        self.pressed_this_frame_inputs.contains(&input)
    }

    #[export]
    pub fn is_keyboard_pressed_this_frame(&self, mut source: String) -> bool {
        source.retain(|c| !c.is_whitespace());

        let result = source
            .split("+")
            .map(|input| match input {
                "Shift" => self.pressed_modifiers.has_shift(),
                "Ctrl" => self.pressed_modifiers.has_ctrl(),
                "Alt" => self.pressed_modifiers.has_alt(),
                "Logo" => self.pressed_modifiers.has_logo(),
                key => self.is_source_pressed_this_frame(format!("Keyboard/{}", key)),
            })
            .fold(true, |acc, elem| acc && elem);

        result
    }

    #[export]
    pub fn is_source_pressed_this_frame(&self, source: String) -> bool {
        self.pressed_this_frame_sources.contains(&source)
    }

    #[export]
    pub fn is_released_this_frame(&self, input: String) -> bool {
        self.released_this_frame_inputs.contains(&input)
    }

    #[export]
    pub fn is_source_released_this_frame(&self, source: String) -> bool {
        self.released_this_frame_sources.contains(&source)
    }

    #[export]
    pub fn notify_pressed(&mut self, source: String) -> FruityResult<()> {
        self.pressed_sources.insert(source.clone());
        self.pressed_this_frame_sources.insert(source.clone());

        if let Some(input) = self.input_map.get(&source) {
            if !self.pressed_inputs.contains(input) {
                self.pressed_inputs.insert(input.clone());
                self.pressed_this_frame_inputs.insert(input.to_string());
                self.on_pressed.send(input.clone())?;
            }
        }

        Ok(())
    }

    #[export]
    pub fn notify_released(&mut self, source: String) -> FruityResult<()> {
        self.pressed_sources.remove(&source);
        self.released_this_frame_sources.insert(source.clone());

        if let Some(input) = self.input_map.get(&source) {
            if self.pressed_inputs.contains(input) {
                self.pressed_inputs.remove(input);
                self.released_this_frame_inputs.insert(input.to_string());
                self.on_released.send(input.clone())?;
            }
        }

        Ok(())
    }

    #[export]
    pub fn handle_frame_end(&mut self) {
        self.pressed_this_frame_sources.clear();
        self.pressed_this_frame_inputs.clear();
        self.released_this_frame_sources.clear();
        self.released_this_frame_inputs.clear();
    }

    pub fn notify_modifiers(&mut self, modifier: Modifiers) {
        self.pressed_modifiers = modifier;
    }
}
