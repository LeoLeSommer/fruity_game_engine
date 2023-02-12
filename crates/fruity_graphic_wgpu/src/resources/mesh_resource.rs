use fruity_game_engine::any::FruityAny;
use fruity_game_engine::export_impl;
use fruity_game_engine::export_struct;
use fruity_game_engine::resource::Resource;
use fruity_graphic::resources::mesh_resource::MeshResource;
use fruity_graphic::resources::mesh_resource::MeshResourceSettings;
use wgpu::util::DeviceExt;

#[derive(Debug, FruityAny, Resource)]
#[export_struct]
pub struct WgpuMeshResource {
    pub params: MeshResourceSettings,
    pub(crate) vertex_buffer: wgpu::Buffer,
    pub(crate) index_buffer: wgpu::Buffer,
    pub(crate) index_count: usize,
}

impl WgpuMeshResource {
    pub fn new(
        device: &wgpu::Device,
        label: &str,
        params: &MeshResourceSettings,
    ) -> WgpuMeshResource {
        // Create the buffers
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{}_vertex", label)),
            contents: bytemuck::cast_slice(&params.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{}_index", label)),
            contents: bytemuck::cast_slice(&params.indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        WgpuMeshResource {
            params: params.clone(),
            vertex_buffer,
            index_buffer,
            index_count: params.indices.len(),
        }
    }
}

#[export_impl]
impl MeshResource for WgpuMeshResource {}
