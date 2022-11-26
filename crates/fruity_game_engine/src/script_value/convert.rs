use super::ScriptValue;
use crate::FruityResult;
pub use fruity_game_engine_macro::TryFromScriptValue;

/// Traits similar to TryInto for ScriptValue
pub trait TryIntoScriptValue: Sized {
    /// Performs the conversion.
    fn into_script_value(self) -> FruityResult<ScriptValue>;
}

/// Traits similar to TryFrom for ScriptValue
pub trait TryFromScriptValue: Sized {
    /// Performs the conversion.
    fn from_script_value(value: ScriptValue) -> FruityResult<Self>;
}
