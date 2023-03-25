use fruity_ecs::entity::entity_service::EntityServiceSnapshot;
use fruity_game_engine::{
    introspect::IntrospectFields,
    script_value::{
        convert::{TryFromScriptValue, TryIntoScriptValue},
        HashMapScriptObject, ScriptValue,
    },
    FruityError, FruityResult,
};
use json::JsonValue;
use std::{collections::HashMap, io::Read, io::Write, path::Path};

/// Extract the file type from a file path
///
/// # Arguments
/// * `file_path` - The file path
///
pub fn get_file_type_from_path(file_path: &str) -> Option<String> {
    let path = Path::new(file_path);
    Some(path.extension()?.to_str()?.to_string())
}

pub fn read_snapshot_from_json(reader: &mut dyn Read) -> FruityResult<EntityServiceSnapshot> {
    let mut string = String::new();
    reader
        .read_to_string(&mut string)
        .map_err(|err| FruityError::GenericFailure(err.to_string()))?;

    let json_value =
        json::parse(&string).map_err(|err| FruityError::GenericFailure(err.to_string()))?;
    let script_value = json_value_to_script_value(json_value)?;

    EntityServiceSnapshot::from_script_value(script_value)
}

fn json_value_to_script_value(json_value: JsonValue) -> FruityResult<ScriptValue> {
    Ok(match json_value {
        JsonValue::Null => ScriptValue::Null,
        JsonValue::Short(value) => ScriptValue::String(value.to_string()),
        JsonValue::String(value) => ScriptValue::String(value),
        JsonValue::Number(value) => ScriptValue::F64(
            value
                .try_into()
                .map_err(|_| FruityError::GenericFailure(format!("Number out of scope")))?,
        ),
        JsonValue::Boolean(value) => ScriptValue::Bool(value),
        JsonValue::Object(json_value) => ScriptValue::Object(Box::new(HashMapScriptObject {
            class_name: "unknown".to_string(),
            fields: json_value
                .iter()
                .map(|(name, json_value)| {
                    FruityResult::Ok((
                        name.to_string(),
                        json_value_to_script_value(json_value.clone())?,
                    ))
                })
                .try_collect()?,
        })),
        JsonValue::Array(json_value) => ScriptValue::Array(
            json_value
                .into_iter()
                .map(|json_value| json_value_to_script_value(json_value))
                .try_collect()?,
        ),
    })
}

pub fn write_snapshot_to_json<T: Write>(
    writer: &mut T,
    snapshot: EntityServiceSnapshot,
) -> FruityResult<()> {
    let json = script_value_to_json_value(ScriptValue::Array(
        snapshot
            .into_iter()
            .map(|serialized_entity| serialized_entity.into_script_value())
            .try_collect::<Vec<_>>()?,
    ))?;

    json.write(writer)
        .map_err(|err| FruityError::GenericFailure(err.to_string()))
}

fn script_value_to_json_value(script_value: ScriptValue) -> FruityResult<JsonValue> {
    Ok(match script_value {
        ScriptValue::I8(value) => JsonValue::Number(value.into()),
        ScriptValue::I16(value) => JsonValue::Number(value.into()),
        ScriptValue::I32(value) => JsonValue::Number(value.into()),
        ScriptValue::I64(value) => JsonValue::Number(value.into()),
        ScriptValue::ISize(value) => JsonValue::Number(value.into()),
        ScriptValue::U8(value) => JsonValue::Number(value.into()),
        ScriptValue::U16(value) => JsonValue::Number(value.into()),
        ScriptValue::U32(value) => JsonValue::Number(value.into()),
        ScriptValue::U64(value) => JsonValue::Number(value.into()),
        ScriptValue::USize(value) => JsonValue::Number(value.into()),
        ScriptValue::F32(value) => JsonValue::Number(value.into()),
        ScriptValue::F64(value) => JsonValue::Number(value.into()),
        ScriptValue::Bool(value) => JsonValue::Boolean(value),
        ScriptValue::String(value) => JsonValue::String(value),
        ScriptValue::Array(script_value) => JsonValue::Array(
            script_value
                .into_iter()
                .map(|script_value| script_value_to_json_value(script_value))
                .try_collect()?,
        ),
        ScriptValue::Null => JsonValue::Null,
        ScriptValue::Undefined => JsonValue::Null,
        ScriptValue::Future(_) => unimplemented!(),
        ScriptValue::Callback(_) => unimplemented!(),
        ScriptValue::Object(script_value) => {
            <dyn IntrospectFields>::get_field_values(&script_value)?
                .into_iter()
                .map(|(name, script_value)| {
                    FruityResult::Ok((name, script_value_to_json_value(script_value)?))
                })
                .try_collect::<HashMap<String, JsonValue>>()?
                .into()
        }
    })
}
