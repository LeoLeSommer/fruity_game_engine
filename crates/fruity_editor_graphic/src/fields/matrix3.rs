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
use fruity_game_engine::Arc;
use fruity_game_engine::FruityResult;
use fruity_graphic::math::matrix3::Matrix3;

pub fn draw_editor_matrix3(
    _ctx: &mut UIContext,
    name: &str,
    value: Box<dyn ScriptObject>,
    on_update: impl Fn(&UIContext, Box<dyn ScriptObject>) -> FruityResult<()> + Send + Sync + 'static,
) -> FruityResult<UIElement> {
    let value = if let Ok(value) = Matrix3::from_script_value(ScriptValue::Object(value)) {
        value
    } else {
        Matrix3::default()
    };

    let on_update = Arc::new(on_update);
    let on_update_2 = on_update.clone();
    let on_update_3 = on_update.clone();
    let on_update_4 = on_update.clone();
    let on_update_5 = on_update.clone();
    let on_update_6 = on_update.clone();
    let on_update_7 = on_update.clone();
    let on_update_8 = on_update.clone();
    let on_update_9 = on_update.clone();

    Ok(Row {
        children: vec![
            RowItem {
                size: UISize::FillPortion(1.0),
                child: Text {
                    text: name.to_string(),
                    ..Default::default()
                }
                .elem(),
            },
            RowItem {
                size: UISize::FillPortion(0.33),
                child: FloatInput {
                    value: value.0[0][0] as f64,
                    on_change: Arc::new(move |ctx, new_value: f64| {
                        let mut value = value;
                        value.0[0][0] = new_value as f32;
                        on_update(ctx, Box::new(value))
                    }),
                }
                .elem(),
            },
            RowItem {
                size: UISize::FillPortion(0.33),
                child: FloatInput {
                    value: value.0[0][0] as f64,
                    on_change: Arc::new(move |ctx, new_value: f64| {
                        let mut value = value;
                        value.0[0][0] = new_value as f32;
                        on_update_2(ctx, Box::new(value))
                    }),
                }
                .elem(),
            },
            RowItem {
                size: UISize::FillPortion(0.33),
                child: FloatInput {
                    value: value.0[0][0] as f64,
                    on_change: Arc::new(move |ctx, new_value: f64| {
                        let mut value = value;
                        value.0[0][0] = new_value as f32;
                        on_update_3(ctx, Box::new(value))
                    }),
                }
                .elem(),
            },
            RowItem {
                size: UISize::FillPortion(0.33),
                child: FloatInput {
                    value: value.0[1][0] as f64,
                    on_change: Arc::new(move |ctx, new_value: f64| {
                        let mut value = value;
                        value.0[1][0] = new_value as f32;
                        on_update_4(ctx, Box::new(value))
                    }),
                }
                .elem(),
            },
            RowItem {
                size: UISize::FillPortion(0.33),
                child: FloatInput {
                    value: value.0[1][0] as f64,
                    on_change: Arc::new(move |ctx, new_value: f64| {
                        let mut value = value;
                        value.0[1][0] = new_value as f32;
                        on_update_5(ctx, Box::new(value))
                    }),
                }
                .elem(),
            },
            RowItem {
                size: UISize::FillPortion(0.33),
                child: FloatInput {
                    value: value.0[1][0] as f64,
                    on_change: Arc::new(move |ctx, new_value: f64| {
                        let mut value = value;
                        value.0[1][0] = new_value as f32;
                        on_update_6(ctx, Box::new(value))
                    }),
                }
                .elem(),
            },
            RowItem {
                size: UISize::FillPortion(0.33),
                child: FloatInput {
                    value: value.0[2][0] as f64,
                    on_change: Arc::new(move |ctx, new_value: f64| {
                        let mut value = value;
                        value.0[2][0] = new_value as f32;
                        on_update_7(ctx, Box::new(value))
                    }),
                }
                .elem(),
            },
            RowItem {
                size: UISize::FillPortion(0.33),
                child: FloatInput {
                    value: value.0[2][0] as f64,
                    on_change: Arc::new(move |ctx, new_value: f64| {
                        let mut value = value;
                        value.0[2][0] = new_value as f32;
                        on_update_8(ctx, Box::new(value))
                    }),
                }
                .elem(),
            },
            RowItem {
                size: UISize::FillPortion(0.33),
                child: FloatInput {
                    value: value.0[2][0] as f64,
                    on_change: Arc::new(move |ctx, new_value: f64| {
                        let mut value = value;
                        value.0[2][0] = new_value as f32;
                        on_update_9(ctx, Box::new(value))
                    }),
                }
                .elem(),
            },
        ],
        ..Default::default()
    }
    .elem())
}
