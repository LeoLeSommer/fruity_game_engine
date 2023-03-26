use fruity_editor::fields::resource_reference::draw_editor_resource_reference;
use fruity_editor::ui::context::UIContext;
use fruity_editor::ui::elements::UIElement;
use fruity_game_engine::script_value::ScriptObject;
use fruity_game_engine::FruityResult;
use fruity_graphic::resources::shader_resource::ShaderResource;

pub fn draw_editor_shader_reference(
    _ctx: &mut UIContext,
    name: &str,
    value: Box<dyn ScriptObject>,
    on_update: impl Fn(&UIContext, Box<dyn ScriptObject>) -> FruityResult<()> + Send + Sync + 'static,
) -> FruityResult<UIElement> {
    draw_editor_resource_reference::<dyn ShaderResource>(name, value, Box::new(on_update))
}
