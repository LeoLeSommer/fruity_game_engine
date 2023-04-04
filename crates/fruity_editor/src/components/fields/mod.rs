use crate::components::fields::primitive::draw_editor_bool;
use crate::components::fields::primitive::draw_editor_f32;
use crate::components::fields::primitive::draw_editor_f64;
use crate::components::fields::primitive::draw_editor_i16;
use crate::components::fields::primitive::draw_editor_i32;
use crate::components::fields::primitive::draw_editor_i64;
use crate::components::fields::primitive::draw_editor_i8;
use crate::components::fields::primitive::draw_editor_isize;
use crate::components::fields::primitive::draw_editor_string;
use crate::components::fields::primitive::draw_editor_u16;
use crate::components::fields::primitive::draw_editor_u32;
use crate::components::fields::primitive::draw_editor_u64;
use crate::components::fields::primitive::draw_editor_u8;
use crate::components::fields::primitive::draw_editor_usize;
use crate::introspect_editor_service::IntrospectEditorService;
use crate::mutations::mutation_service::MutationService;
use crate::mutations::set_field_mutation::SetFieldMutation;
use crate::ui::context::UIContext;
use crate::ui::elements::display::Text;
use crate::ui::elements::input::Button;
use crate::ui::elements::layout::Collapsible;
use crate::ui::elements::layout::Column;
use crate::ui::elements::UIAlign;
use crate::ui::elements::UIElement;
use crate::ui::elements::UIWidget;
use crate::ui::hooks::use_read_service;
use crate::ui::hooks::use_write_service;
use fruity_game_engine::introspect::IntrospectFields;
use fruity_game_engine::script_value::impl_containers::ScriptValueHashMap;
use fruity_game_engine::script_value::ScriptObject;
use fruity_game_engine::script_value::ScriptValue;
use fruity_game_engine::Arc;
use fruity_game_engine::FruityResult;
use std::collections::HashMap;
use std::ops::Deref;

pub mod primitive;

pub fn edit_introspect_fields(
    ctx: &mut UIContext,
    introspect_object: Box<dyn ScriptObject>,
) -> FruityResult<UIElement> {
    let fields_edit = <dyn IntrospectFields>::get_field_values(&introspect_object)?
        .into_iter()
        .map(move |(name, previous_value)| {
            let introspect_object = introspect_object.duplicate();

            field_editor(
                ctx,
                &name.clone(),
                previous_value.clone(),
                Box::new(move |ctx, new_value| {
                    let mut mutation_service = use_write_service::<MutationService>(ctx);

                    mutation_service.push_action(SetFieldMutation::new(
                        introspect_object.duplicate(),
                        name.clone(),
                        new_value,
                    ))
                }),
            )
        })
        .try_collect::<Vec<_>>()?;

    Ok(Column {
        children: fields_edit,
        align: UIAlign::Start,
    }
    .elem())
}

pub fn field_editor(
    ctx: &mut UIContext,
    name: &str,
    value: ScriptValue,
    on_update: Box<dyn Fn(&UIContext, ScriptValue) -> FruityResult<()> + Send + Sync>,
) -> FruityResult<UIElement> {
    match value {
        ScriptValue::U8(value) => draw_editor_u8(name, ScriptValue::U8(value), on_update),
        ScriptValue::U16(value) => draw_editor_u16(name, ScriptValue::U16(value), on_update),
        ScriptValue::U32(value) => draw_editor_u32(name, ScriptValue::U32(value), on_update),
        ScriptValue::U64(value) => draw_editor_u64(name, ScriptValue::U64(value), on_update),
        ScriptValue::USize(value) => draw_editor_usize(name, ScriptValue::USize(value), on_update),
        ScriptValue::I8(value) => draw_editor_i8(name, ScriptValue::I8(value), on_update),
        ScriptValue::I16(value) => draw_editor_i16(name, ScriptValue::I16(value), on_update),
        ScriptValue::I32(value) => draw_editor_i32(name, ScriptValue::I32(value), on_update),
        ScriptValue::I64(value) => draw_editor_i64(name, ScriptValue::I64(value), on_update),
        ScriptValue::ISize(value) => draw_editor_isize(name, ScriptValue::ISize(value), on_update),
        ScriptValue::F32(value) => draw_editor_f32(name, ScriptValue::F32(value), on_update),
        ScriptValue::F64(value) => draw_editor_f64(name, ScriptValue::F64(value), on_update),
        ScriptValue::Bool(value) => draw_editor_bool(name, ScriptValue::Bool(value), on_update),
        ScriptValue::String(value) => {
            draw_editor_string(name, ScriptValue::String(value), on_update)
        }
        ScriptValue::Object(value) => {
            let introspect_editor_service = use_read_service::<IntrospectEditorService>(ctx);

            let type_id = value.deref().type_id();
            if let Some(field_editor) = introspect_editor_service.get_field_editor(type_id) {
                field_editor(
                    ctx,
                    name,
                    value,
                    Box::new(move |ctx, value| on_update(ctx, ScriptValue::Object(value))),
                )
            } else {
                Ok(Text {
                    text: name.to_string(),
                    ..Default::default()
                }
                .elem())
            }
        }
        ScriptValue::Array(elems) => {
            let mut children = Vec::new();
            let elems_2 = elems.clone();

            let on_update = Arc::new(on_update);
            for (index, value) in elems.iter().enumerate() {
                let elems = elems.clone();
                let on_update = on_update.clone();
                children.push(field_editor(
                    ctx,
                    &index.to_string(),
                    value.clone(),
                    Box::new(move |ctx, value| {
                        let mut elems = elems.clone();
                        let _ = std::mem::replace(&mut elems[index], value);
                        on_update(ctx, ScriptValue::Array(elems))
                    }),
                )?)
            }

            let on_update = Arc::new(on_update);
            children.push(
                Button {
                    label: "+".to_string(),
                    on_click: Arc::new(move |ctx| {
                        let mut current_value = elems_2.clone();
                        current_value.push(ScriptValue::Object(Box::new(ScriptValueHashMap {
                            class_name: "unknown".to_string(),
                            fields: HashMap::new(),
                        })));

                        on_update(ctx, ScriptValue::Array(current_value))
                    }),
                    ..Default::default()
                }
                .elem(),
            );

            Ok(Collapsible {
                key: name.to_string(),
                title: name.to_string(),
                child: Column {
                    children,
                    align: UIAlign::Start,
                }
                .elem(),
                ..Default::default()
            }
            .elem())
        }
        _ => Ok(Text {
            text: name.to_string(),
            ..Default::default()
        }
        .elem()),
    }
}
