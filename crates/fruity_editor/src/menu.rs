use fruity_game_engine::FruityResult;

use crate::mutations::mutation_service::MutationService;
use crate::ui::context::UIContext;
use crate::ui::hooks::use_read_service;
use crate::ui::hooks::use_write_service;

pub fn is_undo_enabled(ctx: &UIContext) -> FruityResult<bool> {
    let mutation_service = use_read_service::<MutationService>(ctx);
    Ok(mutation_service.can_undo())
}

pub fn undo(ctx: &UIContext) -> FruityResult<()> {
    let mut mutation_service = use_write_service::<MutationService>(ctx);
    mutation_service.undo()
}

pub fn is_redo_enabled(ctx: &UIContext) -> FruityResult<bool> {
    let mutation_service = use_read_service::<MutationService>(ctx);
    Ok(mutation_service.can_redo())
}

pub fn redo(ctx: &UIContext) -> FruityResult<()> {
    let mut mutation_service = use_write_service::<MutationService>(ctx);
    mutation_service.redo()
}
