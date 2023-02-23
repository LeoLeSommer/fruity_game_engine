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
use fruity_game_engine::resource::resource_reference::ResourceReference;
use fruity_game_engine::resource::Resource;
use fruity_game_engine::script_value::convert::{TryFromScriptValue, TryIntoScriptValue};
use fruity_game_engine::script_value::ScriptValue;
use fruity_game_engine::signal::Signal;
use fruity_game_engine::FruityError;
use fruity_game_engine::{export, export_trait};
use fruity_game_engine::{typescript, FruityResult};
use maplit::hashmap;
use std::collections::HashMap;

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

impl TryIntoScriptValue for MaterialParam {
    fn into_script_value(self) -> FruityResult<ScriptValue> {
        Ok(match self {
            MaterialParam::Uint(value) => hashmap! {
                "type".to_string() => "uint".to_string().into_script_value()?,
                "value".to_string() => value.into_script_value()?,
            }
            .into_script_value()?,
            MaterialParam::Int(value) => hashmap! {
                "type".to_string() => "int".to_string().into_script_value()?,
                "value".to_string() => value.into_script_value()?,
            }
            .into_script_value()?,
            MaterialParam::Float(value) => hashmap! {
                "type".to_string() => "float".to_string().into_script_value()?,
                "value".to_string() => value.into_script_value()?,
            }
            .into_script_value()?,
            MaterialParam::Vector2D(value) => hashmap! {
                "type".to_string() => "vector2d".to_string().into_script_value()?,
                "value".to_string() => value.into_script_value()?,
            }
            .into_script_value()?,
            MaterialParam::Color(value) => hashmap! {
                "type".to_string() => "color".to_string().into_script_value()?,
                "value".to_string() => value.into_script_value()?,
            }
            .into_script_value()?,
            MaterialParam::Rect {
                bottom_left,
                top_right,
            } => hashmap! {
                "type".to_string() => "rect".to_string().into_script_value()?,
                "value".to_string() => hashmap! {
                    "bottomLeft".to_string() => bottom_left.into_script_value()?,
                    "topRight".to_string() => top_right.into_script_value()?,
                }.into_script_value()?,
            }
            .into_script_value()?,
            MaterialParam::Matrix4(value) => hashmap! {
                "type".to_string() => "matrix4".to_string().into_script_value()?,
                "value".to_string() => value.into_script_value()?,
            }
            .into_script_value()?,
        })
    }
}

impl TryFromScriptValue for MaterialParam {
    fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
        match &value {
            ScriptValue::Object(script_object) => {
                let field_names = script_object.get_field_names()?;

                if field_names.contains(&"type".to_string())
                    && field_names.contains(&"value".to_string())
                {
                    let ty = String::from_script_value(script_object.get_field_value("type")?)?;
                    let ty = ty.as_str();
                    let value = script_object.get_field_value("value")?;

                    match ty {
                        "uint" => Ok(MaterialParam::Uint(u32::from_script_value(value)?)),
                        "int" => Ok(MaterialParam::Int(i32::from_script_value(value)?)),
                        "float" => Ok(MaterialParam::Float(f32::from_script_value(value)?)),
                        "vector2d" => {
                            Ok(MaterialParam::Vector2D(Vector2D::from_script_value(value)?))
                        }
                        "color" => Ok(MaterialParam::Color(Color::from_script_value(value)?)),
                        "rect" => {
                            if let ScriptValue::Object(value_object) = &value {
                                let field_names = value_object.get_field_names()?;

                                if field_names.contains(&"bottomLeft".to_string())
                                    && field_names.contains(&"topRight".to_string())
                                {
                                    let bottom_left = Vector2D::from_script_value(
                                        script_object.get_field_value("bottomLeft")?,
                                    )?;

                                    let top_right = Vector2D::from_script_value(
                                        script_object.get_field_value("topRight")?,
                                    )?;

                                    Ok(MaterialParam::Rect {
                                        bottom_left,
                                        top_right,
                                    })
                                } else {
                                    Err(FruityError::InvalidArg(format!(
                                        "Couldn't convert {:?} to MaterialParam",
                                        &value
                                    )))
                                }
                            } else {
                                Err(FruityError::InvalidArg(format!(
                                    "Couldn't convert {:?} to MaterialParam",
                                    &value
                                )))
                            }
                        }
                        "matrix4" => Ok(MaterialParam::Matrix4(Matrix4::from_script_value(value)?)),
                        _ => Err(FruityError::InvalidArg(format!(
                            "Couldn't convert {:?} to MaterialParam",
                            &value
                        ))),
                    }
                } else {
                    Err(FruityError::InvalidArg(format!(
                        "Couldn't convert {:?} to MaterialParam",
                        &value
                    )))
                }
            }
            _ => Err(FruityError::InvalidArg(format!(
                "Couldn't convert {:?} to MaterialParam",
                value
            ))),
        }
    }
}

#[export_trait]
pub trait GraphicService: Resource {
    #[export]
    fn start_draw(&mut self);

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
    fn on_before_draw_end(&self) -> &Signal<()>;
    fn on_after_draw_end(&self) -> &Signal<()>;
}
