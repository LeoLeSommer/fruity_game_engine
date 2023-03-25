use crate::ui::context::UIContext;
use crate::ui::elements::display::Text;
use crate::ui::elements::input::Checkbox;
use crate::ui::elements::input::FloatInput;
use crate::ui::elements::input::Input;
use crate::ui::elements::input::IntegerInput;
use crate::ui::elements::layout::Row;
use crate::ui::elements::layout::RowItem;
use crate::ui::elements::UIElement;
use crate::ui::elements::UISize;
use crate::ui::elements::UIWidget;
use fruity_game_engine::script_value::convert::TryFromScriptValue;
use fruity_game_engine::script_value::convert::TryIntoScriptValue;
use fruity_game_engine::script_value::ScriptValue;
use fruity_game_engine::FruityResult;
use std::sync::Arc;

macro_rules! impl_int_for_editable_component {
    ( $fn_name:ident, $type:ident ) => {
        pub fn $fn_name(
            name: &str,
            value: ScriptValue,
            on_update: impl Fn(&UIContext, ScriptValue) -> FruityResult<()> + Send + Sync + 'static,
        ) -> FruityResult<UIElement> {
            let value = if let Ok(value) = $type::from_script_value(value) {
                value
            } else {
                $type::default()
            };

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
                        size: UISize::Fill,
                        child: IntegerInput {
                            value: value as i64,
                            on_change: Arc::new(move |ctx, value| {
                                on_update(ctx, (value as $type).into_script_value()?)
                            }),
                        }
                        .elem(),
                    },
                ],
                ..Default::default()
            }
            .elem())
        }
    };
}

impl_int_for_editable_component!(draw_editor_u8, u8);
impl_int_for_editable_component!(draw_editor_u16, u16);
impl_int_for_editable_component!(draw_editor_u32, u32);
impl_int_for_editable_component!(draw_editor_u64, u64);
impl_int_for_editable_component!(draw_editor_usize, usize);
impl_int_for_editable_component!(draw_editor_i8, i8);
impl_int_for_editable_component!(draw_editor_i16, i16);
impl_int_for_editable_component!(draw_editor_i32, i32);
impl_int_for_editable_component!(draw_editor_i64, i64);
impl_int_for_editable_component!(draw_editor_isize, isize);

macro_rules! impl_float_for_editable_component {
    ( $fn_name:ident, $type:ident ) => {
        pub fn $fn_name(
            name: &str,
            value: ScriptValue,
            on_update: impl Fn(&UIContext, ScriptValue) -> FruityResult<()> + Send + Sync + 'static,
        ) -> FruityResult<UIElement> {
            let value = if let Ok(value) = $type::from_script_value(value) {
                value
            } else {
                $type::default()
            };

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
                        size: UISize::Fill,
                        child: FloatInput {
                            value: value as f64,
                            on_change: Arc::new(move |ctx, value| {
                                on_update(ctx, (value as $type).into_script_value()?)
                            }),
                        }
                        .elem(),
                    },
                ],
                ..Default::default()
            }
            .elem())
        }
    };
}

impl_float_for_editable_component!(draw_editor_f32, f32);
impl_float_for_editable_component!(draw_editor_f64, f64);

pub fn draw_editor_bool(
    name: &str,
    value: ScriptValue,
    on_update: impl Fn(&UIContext, ScriptValue) -> FruityResult<()> + Send + Sync + 'static,
) -> FruityResult<UIElement> {
    let value = if let Ok(value) = bool::from_script_value(value) {
        value
    } else {
        bool::default()
    };

    Ok(Checkbox {
        label: name.to_string(),
        value: value,
        on_change: Arc::new(move |ctx, value| on_update(ctx, value.into_script_value()?)),
    }
    .elem())
}

pub fn draw_editor_string(
    name: &str,
    value: ScriptValue,
    on_update: impl Fn(&UIContext, ScriptValue) -> FruityResult<()> + Send + Sync + 'static,
) -> FruityResult<UIElement> {
    let value = if let Ok(value) = String::from_script_value(value) {
        value
    } else {
        String::default()
    };

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
                size: UISize::Fill,
                child: Input {
                    value: value,
                    on_change: Arc::new(move |ctx, value: &str| {
                        on_update(ctx, value.to_string().into_script_value()?)
                    }),
                    ..Default::default()
                }
                .elem(),
            },
        ],
        ..Default::default()
    }
    .elem())
}
