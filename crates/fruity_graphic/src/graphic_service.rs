use crate::math::matrix4::Matrix4;
use crate::math::Color;
use crate::resources::material_resource::MaterialResource;
use crate::resources::material_resource::MaterialResourceSettings;
use crate::resources::mesh_resource::MeshResource;
use crate::resources::mesh_resource::MeshResourceSettings;
use crate::resources::shader_resource::ShaderResource;
use crate::resources::shader_resource::ShaderResourceSettings;
use crate::resources::texture_resource::TextureResource;
use crate::resources::texture_resource::TextureResourceSettings;
use crate::Vector2D;
use fruity_game_engine::any::FruityAny;
use fruity_game_engine::introspect::IntrospectFields;
use fruity_game_engine::introspect::IntrospectMethods;
use fruity_game_engine::resource::ResourceReference;
use fruity_game_engine::script_value::ScriptValue;
use fruity_game_engine::script_value::TryFromScriptValue;
use fruity_game_engine::script_value::TryIntoScriptValue;
use fruity_game_engine::FruityError;
use fruity_game_engine::{export, export_trait};
use fruity_game_engine::{typescript, FruityResult};
use maplit::hashmap;
use std::collections::HashMap;

#[derive(Debug, FruityAny)]
#[typescript(
    "type MaterialParam =
  | { type: 'uint', value: number }
  | { type: 'int', value: number }
  | { type: 'float', value: number }
  | { type: 'vector2d', value: Vector2D }
  | { type: 'color', value: Color }
  | { type: 'rect', value: {
    bottomLeft: Vector2D,
    topRight: Vector2D,
  } }
  | { type: 'matrix4', value: Matrix4 }"
)]
pub enum MaterialParam {
    Uint(u32),
    Int(i32),
    Float(f32),
    Vector2D(Vector2D),
    Color(Color),
    Rect {
        bottom_left: Vector2D,
        top_right: Vector2D,
    },
    Matrix4(Matrix4),
}

impl Default for MaterialParam {
    fn default() -> Self {
        MaterialParam::Uint(0)
    }
}

impl IntrospectFields for MaterialParam {
    fn is_static(&self) -> FruityResult<bool> {
        Ok(true)
    }

    fn get_class_name(&self) -> FruityResult<String> {
        Ok("MaterialParam".to_string())
    }

    fn get_field_names(&self) -> FruityResult<Vec<String>> {
        Ok(vec!["type".to_string(), "value".to_string()])
    }

    fn set_field_value(&mut self, name: &str, value: ScriptValue) -> FruityResult<()> {
        match name {
            "type" => match String::from_script_value(value)?.as_str() {
                "uint" => {
                    *self = MaterialParam::Uint(0);
                }
                "int" => {
                    *self = MaterialParam::Int(0);
                }
                "float" => {
                    *self = MaterialParam::Float(0.0);
                }
                "vector2d" => {
                    *self = MaterialParam::Vector2D(Vector2D::default());
                }
                "color" => {
                    *self = MaterialParam::Color(Color::default());
                }
                "rect" => {
                    *self = MaterialParam::Rect {
                        bottom_left: Vector2D::default(),
                        top_right: Vector2D::default(),
                    };
                }
                _ => unreachable!(),
            },
            "value" => match self {
                MaterialParam::Uint(self_value) => {
                    *self_value = u32::from_script_value(value)?;
                }
                MaterialParam::Int(self_value) => {
                    *self_value = i32::from_script_value(value)?;
                }
                MaterialParam::Float(self_value) => {
                    *self_value = f32::from_script_value(value)?;
                }
                MaterialParam::Vector2D(self_value) => {
                    *self_value = Vector2D::from_script_value(value)?;
                }
                MaterialParam::Color(self_value) => {
                    *self_value = Color::from_script_value(value)?;
                }
                MaterialParam::Rect {
                    bottom_left,
                    top_right,
                } => match value {
                    ScriptValue::Object(script_object) => {
                        *bottom_left = Vector2D::from_script_value(
                            script_object.get_field_value("bottom_left")?,
                        )?;
                        *top_right = Vector2D::from_script_value(
                            script_object.get_field_value("top_right")?,
                        )?;
                    }
                    _ => unreachable!(),
                },
                MaterialParam::Matrix4(self_value) => {
                    *self_value = Matrix4::from_script_value(value)?
                }
            },
            _ => unreachable!(),
        };

        FruityResult::Ok(())
    }

    fn get_field_value(&self, name: &str) -> FruityResult<ScriptValue> {
        match name {
            "type" => match self {
                MaterialParam::Uint(_) => Ok("uint".into_script_value()?),
                MaterialParam::Int(_) => Ok("int".into_script_value()?),
                MaterialParam::Float(_) => Ok("float".into_script_value()?),
                MaterialParam::Vector2D(_) => Ok("vector2d".into_script_value()?),
                MaterialParam::Color(_) => Ok("color".into_script_value()?),
                MaterialParam::Rect {
                    bottom_left: _,
                    top_right: _,
                } => Ok("rect".into_script_value()?),
                MaterialParam::Matrix4(_) => Ok("matrix4".into_script_value()?),
            },
            "value" => match self {
                MaterialParam::Uint(value) => Ok(value.into_script_value()?),
                MaterialParam::Int(value) => Ok(value.into_script_value()?),
                MaterialParam::Float(value) => Ok(value.into_script_value()?),
                MaterialParam::Vector2D(value) => Ok(value.into_script_value()?),
                MaterialParam::Color(value) => Ok(value.into_script_value()?),
                MaterialParam::Rect {
                    bottom_left,
                    top_right,
                } => {
                    let hashmap = hashmap! {
                        "bottom_left".to_string() => bottom_left.clone(),
                        "top_right".to_string() => top_right.clone(),
                    };

                    Ok(hashmap.into_script_value()?)
                }
                MaterialParam::Matrix4(value) => Ok(value.into_script_value()?),
            },
            _ => unreachable!(),
        }
    }
}

impl IntrospectMethods for MaterialParam {
    fn get_const_method_names(&self) -> FruityResult<Vec<String>> {
        Ok(vec![])
    }

    fn call_const_method(&self, _name: &str, _args: Vec<ScriptValue>) -> FruityResult<ScriptValue> {
        unreachable!()
    }

    fn get_mut_method_names(&self) -> FruityResult<Vec<String>> {
        Ok(vec![])
    }

    fn call_mut_method(
        &mut self,
        _name: &str,
        _args: Vec<ScriptValue>,
    ) -> FruityResult<ScriptValue> {
        unreachable!()
    }
}

impl TryIntoScriptValue for MaterialParam {
    fn into_script_value(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::Object(Box::new(self)))
    }
}

impl TryFromScriptValue for MaterialParam {
    fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
        use std::ops::Deref;

        match value {
            ScriptValue::Object(value) => match value.downcast::<Self>() {
                Ok(value) => Ok(*value),
                Err(value) => Err(FruityError::InvalidArg(format!(
                    "Couldn't convert a {} to {}",
                    value.deref().get_type_name(),
                    std::any::type_name::<Self>()
                ))),
            },
            value => Err(FruityError::InvalidArg(format!(
                "Couldn't convert {:?} to native object",
                value
            ))),
        }
    }
}

#[export_trait]
pub trait GraphicService: IntrospectFields + IntrospectMethods + Send + Sync {
    #[export]
    fn start_draw(&mut self) -> FruityResult<()>;

    #[export]
    fn end_draw(&mut self);

    #[export]
    fn render_scene(
        &self,
        view_proj: Matrix4,
        background_color: Color,
        target: Option<ResourceReference<dyn TextureResource>>,
    );

    #[export]
    fn get_camera_transform(&self) -> Matrix4;

    #[export]
    fn resize(&mut self, width: u32, height: u32);

    #[export]
    fn world_position_to_viewport_position(&self, pos: Vector2D) -> (u32, u32);

    #[export]
    fn viewport_position_to_world_position(&self, x: u32, y: u32) -> Vector2D;

    #[export]
    fn get_cursor_position(&self) -> Vector2D;

    #[export]
    fn is_cursor_hover_scene(&self) -> bool;

    #[export]
    fn get_viewport_offset(&self) -> (u32, u32);

    #[export]
    fn set_viewport_offset(&self, x: u32, y: u32);

    #[export]
    fn get_viewport_size(&self) -> (u32, u32);

    #[export]
    fn set_viewport_size(&self, x: u32, y: u32);

    fn draw_mesh(
        &self,
        identifier: u64,
        mesh: ResourceReference<dyn MeshResource>,
        material: ResourceReference<dyn MaterialResource>,
        params: HashMap<String, MaterialParam>,
        z_index: i32,
    );
    fn create_mesh_resource(
        &self,
        identifier: &str,
        params: MeshResourceSettings,
    ) -> FruityResult<Box<dyn MeshResource>>;
    fn create_shader_resource(
        &self,
        identifier: &str,
        contents: String,
        params: ShaderResourceSettings,
    ) -> FruityResult<Box<dyn ShaderResource>>;
    fn create_texture_resource(
        &self,
        identifier: &str,
        contents: &[u8],
        params: TextureResourceSettings,
    ) -> FruityResult<Box<dyn TextureResource>>;
    fn create_material_resource(
        &self,
        identifier: &str,
        params: MaterialResourceSettings,
    ) -> FruityResult<Box<dyn MaterialResource>>;
}
