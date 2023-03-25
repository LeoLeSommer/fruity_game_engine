use crate::mutations::mutation::Mutation;
use fruity_game_engine::script_value::ScriptObject;
use fruity_game_engine::script_value::ScriptValue;
use fruity_game_engine::FruityResult;

pub struct SetFieldMutation {
    pub target: Box<dyn ScriptObject>,
    pub field: String,
    pub new_value: ScriptValue,
    previous_value: Option<ScriptValue>,
}

impl SetFieldMutation {
    pub fn new(target: Box<dyn ScriptObject>, field: String, new_value: ScriptValue) -> Self {
        Self {
            target,
            field,
            new_value,
            previous_value: None,
        }
    }
}

impl Mutation for SetFieldMutation {
    fn apply(&mut self) -> FruityResult<()> {
        self.previous_value = Some(self.target.get_field_value(&self.field)?);
        self.target
            .set_field_value(&self.field, self.new_value.clone())
    }

    fn undo(&mut self) -> FruityResult<()> {
        // Modify the field value from the previous stored value
        if let Some(previous_value) = &self.previous_value {
            self.target
                .set_field_value(&self.field, previous_value.clone())?;
        }

        Ok(())
    }
}
