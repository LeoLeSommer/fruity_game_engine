use fruity_editor::ui::context::UIContext;
use fruity_editor::ui::elements::display::Text;
use fruity_editor::ui::elements::input::FloatInput;
use fruity_editor::ui::elements::layout::Row;
use fruity_editor::ui::elements::layout::RowItem;
use fruity_editor::ui::elements::UIElement;
use fruity_editor::ui::elements::UISize;
use fruity_editor::ui::elements::UIWidget;
use fruity_game_engine::script_value::convert::TryFromScriptValue;
use fruity_game_engine::script_value::ScriptObject;
use fruity_game_engine::script_value::ScriptValue;
use fruity_game_engine::FruityResult;
use fruity_graphic::math::vector2d::Vector2D;
use std::sync::Arc;

pub fn draw_editor_vector_2d(
    _ctx: &mut UIContext,
    name: &str,
    value: Box<dyn ScriptObject>,
    on_update: impl Fn(&UIContext, Box<dyn ScriptObject>) -> FruityResult<()> + Send + Sync + 'static,
) -> FruityResult<UIElement> {
    let value = if let Ok(value) = Vector2D::from_script_value(ScriptValue::Object(value)) {
        value
    } else {
        Vector2D::default()
    };

    let on_update = Arc::new(on_update);
    let on_update_2 = on_update.clone();
    Ok(Row {
        children: vec![
            RowItem {
                size: UISize::Units(40.0),
                child: Text {
                    text: name.to_string(),
                    ..Default::default()
                }
                .elem(),
            },
            RowItem {
                size: UISize::FillPortion(0.5),
                child: FloatInput {
                    value: value.x as f64,
                    on_change: Arc::new(move |ctx, new_x_value: f64| {
                        let mut value = value;
                        value.x = new_x_value as f32;
                        on_update(ctx, Box::new(value))
                    }),
                }
                .elem(),
            },
            RowItem {
                size: UISize::FillPortion(0.5),
                child: FloatInput {
                    value: value.y as f64,
                    on_change: Arc::new(move |ctx, new_y_value: f64| {
                        let mut value = value;
                        value.y = new_y_value as f32;
                        on_update_2(ctx, Box::new(value))
                    }),
                }
                .elem(),
                ..Default::default()
            },
        ],
        ..Default::default()
    }
    .elem())
}
