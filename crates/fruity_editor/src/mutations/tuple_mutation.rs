use crate::mutations::mutation::Mutation;
use fruity_game_engine::FruityResult;

impl<T1: Mutation, T2: Mutation> Mutation for (T1, T2) {
    fn apply(&mut self) -> FruityResult<()> {
        self.0.apply()?;
        self.1.apply()?;

        Ok(())
    }

    fn undo(&mut self) -> FruityResult<()> {
        self.0.undo()?;
        self.1.undo()?;

        Ok(())
    }
}
