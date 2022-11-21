use crate::serialize::AnyValue;
use linked_hash_map::LinkedHashMap;
use std::collections::HashMap;
use std::io::Read;
use std::io::Write;
use yaml_rust::Yaml;
use yaml_rust::YamlEmitter;
use yaml_rust::YamlLoader;

/// Serialize a [’AnyValue’] as a yaml file
///
/// # Arguments
/// * `reader` - The read io stream
///
pub fn serialize_yaml(writer: &mut dyn Write, serialized: &AnyValue) -> Result<(), std::io::Error> {
    let yaml = intern_serialize_yaml(serialized);

    let mut write_buf = String::new();
    let mut emitter = YamlEmitter::new(&mut write_buf);
    emitter.dump(&yaml).unwrap();

    writer.write_all(write_buf.as_bytes())?;
    Ok(())
}

/// Deserialize a [’AnyValue’] from a yaml file
///
/// # Arguments
/// * `reader` - The read io stream
///
pub fn deserialize_yaml(reader: &mut dyn Read) -> Option<AnyValue> {
    let mut buffer = String::new();
    if let Err(err) = reader.read_to_string(&mut buffer) {
        log::error!("{}", err.to_string());
        return None;
    }

    let docs = YamlLoader::load_from_str(&buffer).unwrap();
    let yaml = &docs[0];

    intern_deserialize_yaml(yaml)
}

fn intern_serialize_yaml(serialized: &AnyValue) -> Yaml {
    match serialized {
        AnyValue::I8(value) => Yaml::Integer(*value as i64),
        AnyValue::I16(value) => Yaml::Integer(*value as i64),
        AnyValue::I32(value) => Yaml::Integer(*value as i64),
        AnyValue::I64(value) => Yaml::Integer(*value as i64),
        AnyValue::ISize(value) => Yaml::Integer(*value as i64),
        AnyValue::U8(value) => Yaml::Integer(*value as i64),
        AnyValue::U16(value) => Yaml::Integer(*value as i64),
        AnyValue::U32(value) => Yaml::Integer(*value as i64),
        AnyValue::U64(value) => Yaml::Integer(*value as i64),
        AnyValue::USize(value) => Yaml::Integer(*value as i64),
        AnyValue::F32(value) => Yaml::Real(value.to_string()),
        AnyValue::F64(value) => Yaml::Real(value.to_string()),
        AnyValue::Bool(value) => Yaml::Boolean(*value),
        AnyValue::String(value) => Yaml::String(value.to_string()),
        AnyValue::Array(value) => {
            let elements = value
                .iter()
                .map(|elem| intern_serialize_yaml(elem))
                .collect::<Vec<_>>();

            Yaml::Array(elements)
        }
        AnyValue::Null => Yaml::Null,
        AnyValue::Iterator(_) => Yaml::BadValue,
        AnyValue::Closure(_) => Yaml::BadValue,
        AnyValue::Object { class_name, fields } => {
            let mut hashmap = LinkedHashMap::<Yaml, Yaml>::new();
            let field_hashmap = {
                let mut hashmap = LinkedHashMap::<Yaml, Yaml>::new();

                fields.iter().for_each(|(key, value)| {
                    let key = Yaml::String(key.clone());
                    hashmap.insert(key, intern_serialize_yaml(value));
                });

                hashmap
            };

            hashmap.insert(
                Yaml::String("class_name".to_string()),
                Yaml::String(class_name.clone()),
            );

            hashmap.insert(
                Yaml::String("fields".to_string()),
                Yaml::Hash(field_hashmap),
            );

            Yaml::Hash(hashmap)
        }
        AnyValue::NativeObject(_) => Yaml::BadValue,
    }
}

fn intern_deserialize_yaml(yaml: &Yaml) -> Option<AnyValue> {
    match yaml {
        Yaml::Real(string) => match string.parse::<f64>() {
            Ok(value) => Some(AnyValue::F64(value)),
            Err(_) => None,
        },
        Yaml::Integer(value) => Some(AnyValue::I64(*value)),
        Yaml::String(value) => Some(AnyValue::String(value.clone())),
        Yaml::Boolean(value) => Some(AnyValue::Bool(*value)),
        Yaml::Array(array) => {
            let settings_array = array
                .iter()
                .filter_map(|elem| intern_deserialize_yaml(elem))
                .collect::<Vec<_>>();

            Some(AnyValue::Array(settings_array))
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

                    Some(AnyValue::Object {
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
