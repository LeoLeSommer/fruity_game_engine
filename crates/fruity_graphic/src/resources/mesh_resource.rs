use crate::math::vector3d::Vector3d;
use crate::Vector2d;
use fruity_game_engine::any::FruityAny;
use fruity_game_engine::resource::Resource;
use fruity_game_engine::script_value::convert::{TryFromScriptValue, TryIntoScriptValue};

#[repr(C)]
#[derive(
    Copy,
    Clone,
    Default,
    TryFromScriptValue,
    TryIntoScriptValue,
    FruityAny,
    Debug,
    bytemuck::Pod,
    bytemuck::Zeroable,
)]
pub struct Vertex {
    pub position: Vector3d,
    pub tex_coords: Vector2d,
    pub normal: Vector3d,
}

pub trait MeshResource: Resource {}

#[derive(Debug, Clone, TryFromScriptValue, TryIntoScriptValue, Default)]
pub struct MeshResourceSettings {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}
