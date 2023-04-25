use crate::math::vector3d::Vector3D;
use crate::Vector2D;
use fruity_game_engine::any::FruityAny;
use fruity_game_engine::introspect::{IntrospectFields, IntrospectMethods};
use fruity_game_engine::{export_impl, export_struct, export_trait};

#[repr(C)]
#[derive(Copy, Clone, Default, FruityAny, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[export_struct]
pub struct Vertex {
    pub position: Vector3D,
    pub tex_coords: Vector2D,
    pub normal: Vector3D,
}

#[export_impl]
impl Vertex {}

#[export_trait]
pub trait MeshResource: IntrospectFields + IntrospectMethods + Send + Sync {}

#[derive(Debug, Clone, Default, FruityAny)]
#[export_struct]
pub struct MeshResourceSettings {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}

#[export_impl]
impl MeshResourceSettings {}
