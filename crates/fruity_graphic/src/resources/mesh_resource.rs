use crate::math::vector3d::Vector3D;
use crate::Vector2D;
use fruity_game_engine::any::FruityAny;
use fruity_game_engine::export_trait;
use fruity_game_engine::introspect::{IntrospectFields, IntrospectMethods};
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
    pub position: Vector3D,
    pub tex_coords: Vector2D,
    pub normal: Vector3D,
}

#[export_trait]
pub trait MeshResource: IntrospectFields + IntrospectMethods + Send + Sync {}

#[derive(Debug, Clone, TryFromScriptValue, TryIntoScriptValue, Default)]
pub struct MeshResourceSettings {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}
