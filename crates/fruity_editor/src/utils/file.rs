use fruity_ecs::entity::entity_service::EntityServiceSnapshot;
use fruity_game_engine::{ settings::Settings, FruityError, FruityResult};
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
    json_value_to_settings(json_value)
}

fn json_value_to_settings(json_value: JsonValue) -> FruityResult<Settings> {
    Ok(match json_value {
        JsonValue::Null => Settings::Null,
        JsonValue::Short(value) => Settings::String(value.to_string()),
        JsonValue::String(value) => Settings::String(value),
        JsonValue::Number(value) => Settings::F64(
            value
                .try_into()
                .map_err(|_| FruityError::GenericFailure(format!("Number out of scope")))?,
        ),
        JsonValue::Boolean(value) => Settings::Bool(value),
        JsonValue::Object(json_value) => Settings::Object(
            json_value
                .iter()
                .map(|(name, json_value)| {
                    FruityResult::Ok((
                        name.to_string(),
                        json_value_to_settings(json_value.clone())?,
                    ))
                })
                .try_collect()?,
        ),
        JsonValue::Array(json_value) => Settings::Array(
            json_value
                .into_iter()
                .map(|json_value| json_value_to_settings(json_value))
                .try_collect()?,
        ),
    })
}

pub fn write_snapshot_to_json<T: Write>(
    writer: &mut T,
    snapshot: EntityServiceSnapshot,
) -> FruityResult<()> {
    let json = settings_to_json_value(snapshot)?;

    json.write(writer)
        .map_err(|err| FruityError::GenericFailure(err.to_string()))
}

fn settings_to_json_value(settings: Settings) -> FruityResult<JsonValue> {
    Ok(match settings {
        Settings::F64(value) => JsonValue::Number(value.into()),
        Settings::Bool(value) => JsonValue::Boolean(value),
        Settings::String(value) => JsonValue::String(value),
        Settings::Array(settings) => JsonValue::Array(
            settings
                .into_iter()
                .map(|script_value| settings_to_json_value(script_value))
                .try_collect()?,
        ),
        Settings::Null => JsonValue::Null,
        Settings::Object(settings) => settings
            .into_iter()
            .map(|(name, script_value)| {
                FruityResult::Ok((name, settings_to_json_value(script_value)?))
            })
            .try_collect::<HashMap<String, JsonValue>>()?
            .into(),
    })
}
