use super::ScriptValue;
use crate::FruityError;
use crate::FruityResult;
use crate::FruityStatus;
use linked_hash_map::LinkedHashMap;
use std::collections::HashMap;
use std::io::Read;
use std::io::Write;
use yaml_rust::Yaml;
use yaml_rust::YamlEmitter;
use yaml_rust::YamlLoader;

/// Serialize a [’ScriptValue’] as a yaml file
///
/// # Arguments
/// * `reader` - The read io stream
///
pub fn serialize_yaml(writer: &mut dyn Write, serialized: &ScriptValue) -> FruityResult<()> {
    let yaml = intern_serialize_yaml(serialized)?;

    let mut write_buf = String::new();
    let mut emitter = YamlEmitter::new(&mut write_buf);
    emitter.dump(&yaml).map_err(|_| {
        FruityError::new(
            FruityStatus::GenericFailure,
            format!("Failed to write a yaml file"),
        )
    })?;

    writer.write_all(write_buf.as_bytes())?;
    Ok(())
}

/// Deserialize a [’ScriptValue’] from a yaml file
///
/// # Arguments
/// * `reader` - The read io stream
///
pub fn deserialize_yaml(reader: &mut dyn Read) -> FruityResult<ScriptValue> {
    let mut buffer = String::new();

    reader.read_to_string(&mut buffer).map_err(|_| {
        FruityError::new(
            FruityStatus::GenericFailure,
            format!("File couldn't be read"),
        )
    })?;

    let docs = YamlLoader::load_from_str(&buffer).map_err(|_| {
        FruityError::new(FruityStatus::GenericFailure, format!("Incorrect Yaml file"))
    })?;

    let yaml = &docs[0];

    match intern_deserialize_yaml(yaml) {
        Some(result) => Ok(result),
        None => Err(FruityError::new(
            FruityStatus::GenericFailure,
            format!("Incorrect Yaml file"),
        )),
    }
}

fn intern_serialize_yaml(serialized: &ScriptValue) -> FruityResult<Yaml> {
    Ok(match serialized {
        ScriptValue::I8(value) => Yaml::Integer(*value as i64),
        ScriptValue::I16(value) => Yaml::Integer(*value as i64),
        ScriptValue::I32(value) => Yaml::Integer(*value as i64),
        ScriptValue::I64(value) => Yaml::Integer(*value as i64),
        ScriptValue::ISize(value) => Yaml::Integer(*value as i64),
        ScriptValue::U8(value) => Yaml::Integer(*value as i64),
        ScriptValue::U16(value) => Yaml::Integer(*value as i64),
        ScriptValue::U32(value) => Yaml::Integer(*value as i64),
        ScriptValue::U64(value) => Yaml::Integer(*value as i64),
        ScriptValue::USize(value) => Yaml::Integer(*value as i64),
        ScriptValue::F32(value) => Yaml::Real(value.to_string()),
        ScriptValue::F64(value) => Yaml::Real(value.to_string()),
        ScriptValue::Bool(value) => Yaml::Boolean(*value),
        ScriptValue::String(value) => Yaml::String(value.to_string()),
        ScriptValue::Array(value) => {
            let elements = value
                .iter()
                .map(|elem| intern_serialize_yaml(elem))
                .collect::<Vec<_>>();

            Yaml::Array(elements)
        }
        ScriptValue::Null => Yaml::Null,
        ScriptValue::Undefined => Yaml::Null,
        ScriptValue::Iterator(_) => Yaml::BadValue,
        ScriptValue::Callback(_) => Yaml::BadValue,
        ScriptValue::Object(value) => {
            let mut hashmap = LinkedHashMap::<Yaml, Yaml>::new();

            value.get_field_names().into_iter().try_for_each(|name| {
                let field_value = value.get_field_value(&name)?;
                let name = Yaml::String(name.clone());
                hashmap.insert(name, intern_serialize_yaml(&field_value));

                FruityResult::Ok(())
            })?;

            hashmap.insert(
                Yaml::String("class_name".to_string()),
                Yaml::String(value.get_class_name()),
            );

            hashmap.insert(Yaml::String("fields".to_string()), Yaml::Hash(hashmap));

            Yaml::Hash(hashmap)
        }
        ScriptValue::Object(_) => Yaml::BadValue,
    })
}

fn intern_deserialize_yaml(yaml: &Yaml) -> Option<ScriptValue> {
    match yaml {
        Yaml::Real(string) => match string.parse::<f64>() {
            Ok(value) => Some(ScriptValue::F64(value)),
            Err(_) => None,
        },
        Yaml::Integer(value) => Some(ScriptValue::I64(*value)),
        Yaml::String(value) => Some(ScriptValue::String(value.clone())),
        Yaml::Boolean(value) => Some(ScriptValue::Bool(*value)),
        Yaml::Array(array) => {
            let settings_array = array
                .iter()
                .filter_map(|elem| intern_deserialize_yaml(elem))
                .collect::<Vec<_>>();

            Some(ScriptValue::Array(settings_array))
        }
        Yaml::Hash(hashmap) => {
            let class_name_key = Yaml::String("class_name".to_string());
            let class_name = hashmap.get(&class_name_key)?;

            let fields_key = Yaml::String("fields".to_string());
            let fields = hashmap.get(&fields_key)?;

            if let Yaml::String(class_name) = class_name {
                if let Yaml::Hash(hashmap) = fields {
                    let mut fields = HashMap::new();

                    for (key, value) in hashmap {
                        if let Yaml::String(key) = key {
                            if let Some(settings) = intern_deserialize_yaml(value) {
                                fields.insert(key.clone(), settings);
                            }
                        }
                    }

                    Some(ScriptValue::Object {
                        class_name: class_name.to_string(),
                        fields,
                    })
                } else {
                    return None;
                }
            } else {
                return None;
            }
        }
        Yaml::Alias(_) => None,
        Yaml::Null => None,
        Yaml::BadValue => None,
    }
}
