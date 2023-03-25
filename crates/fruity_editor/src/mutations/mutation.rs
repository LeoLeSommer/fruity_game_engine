use fruity_game_engine::FruityResult;

pub trait Mutation: Send + Sync + 'static {
    fn apply(&mut self) -> FruityResult<()>;
    fn undo(&mut self) -> FruityResult<()>;
}
