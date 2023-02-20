use crate::graphic_service::GraphicService;
use crate::math::vector3d::Vector3d;
use crate::resources::material_resource::MaterialResourceSettings;
use crate::resources::material_resource::MaterialSettingsBinding;
use crate::resources::material_resource::MaterialSettingsInstanceAttribute;
use crate::resources::mesh_resource::MeshResourceSettings;
use crate::resources::mesh_resource::Vertex;
use crate::resources::shader_resource::ShaderBinding;
use crate::resources::shader_resource::ShaderBindingGroup;
use crate::resources::shader_resource::ShaderBindingType;
use crate::resources::shader_resource::ShaderBindingVisibility;
use crate::resources::shader_resource::ShaderInstanceAttribute;
use crate::resources::shader_resource::ShaderInstanceAttributeType;
use crate::resources::shader_resource::ShaderResource;
use crate::resources::shader_resource::ShaderResourceSettings;
use crate::Vector2d;
use fruity_game_engine::resource::resource_container::ResourceContainer;
use fruity_game_engine::FruityResult;
use maplit::hashmap;

pub fn load_default_resources(resource_container: ResourceContainer) -> FruityResult<()> {
    load_squad_mesh(resource_container.clone())?;
    load_draw_line_shader(resource_container.clone())?;
    load_draw_line_material(resource_container.clone())?;
    load_draw_dotted_line_shader(resource_container.clone())?;
    load_draw_dotted_line_material(resource_container.clone())?;
    load_draw_rect_shader(resource_container.clone())?;
    load_draw_rect_material(resource_container.clone())?;
    load_draw_arc_shader(resource_container.clone())?;
    load_draw_arc_material(resource_container.clone())?;

    Ok(())
}

pub fn load_squad_mesh(resource_container: ResourceContainer) -> FruityResult<()> {
    let graphic_service = resource_container.require::<dyn GraphicService>();
    let graphic_service = graphic_service.read();

    let resource = graphic_service.create_mesh_resource(
        "Meshes/Squad",
        MeshResourceSettings {
            vertices: vec![
                Vertex {
                    position: Vector3d::new(-0.5, -0.5, 0.0),
                    tex_coords: Vector2d::new(0.0, 1.0),
                    normal: Vector3d::new(0.0, 0.0, -1.0),
                },
                Vertex {
                    position: Vector3d::new(0.5, -0.5, 0.0),
                    tex_coords: Vector2d::new(1.0, 1.0),
                    normal: Vector3d::new(0.0, 0.0, -1.0),
                },
                Vertex {
                    position: Vector3d::new(0.5, 0.5, 0.0),
                    tex_coords: Vector2d::new(1.0, 0.0),
                    normal: Vector3d::new(0.0, 0.0, -1.0),
                },
                Vertex {
                    position: Vector3d::new(-0.5, 0.5, 0.0),
                    tex_coords: Vector2d::new(0.0, 0.0),
                    normal: Vector3d::new(0.0, 0.0, -1.0),
                },
            ],
            indices: vec![0, 1, 2, 3, 0, 2, /* padding */ 0],
        },
    )?;

    resource_container.add("Meshes/Squad", resource);

    Ok(())
}

pub fn load_draw_line_shader(resource_container: ResourceContainer) -> FruityResult<()> {
    let graphic_service = resource_container.require::<dyn GraphicService>();
    let graphic_service = graphic_service.read();

    let code = "
        [[block]]
        struct CameraUniform {
            view_proj: mat4x4<f32>;
        };
        
        [[block]]
        struct RenderSurfaceSizeUniform {
            value: vec2<f32>;
        };
        
        struct VertexInput {
            [[location(0)]] position: vec3<f32>;
            [[location(1)]] tex_coords: vec2<f32>;
            [[location(2)]] normal: vec3<f32>;
        };
        
        struct InstanceInput {
            [[location(5)]] pos1: vec2<f32>;
            [[location(6)]] pos2: vec2<f32>;
            [[location(7)]] width: u32;
            [[location(8)]] color: vec4<f32>;
        };
        
        struct VertexOutput {
            [[builtin(position)]] position: vec4<f32>;
            [[location(0)]] color: vec4<f32>;
        };

        [[group(0), binding(0)]]
        var<uniform> camera: CameraUniform;

        [[group(1), binding(0)]]
        var<uniform> render_surface_size: RenderSurfaceSizeUniform;

        [[stage(vertex)]]
        fn main(
            model: VertexInput,
            instance: InstanceInput,
        ) -> VertexOutput {
            var diff = camera.view_proj * (vec4<f32>(instance.pos2, 0.0, 1.0) - vec4<f32>(instance.pos1, 0.0, 0.0));
            let x_scale = f32(instance.width) / render_surface_size.value.x;
            let y_scale = f32(instance.width) / render_surface_size.value.y;
            let normal = normalize(vec2<f32>(-diff.y, diff.x));
            let scaled_normal = vec2<f32>(normal.x * x_scale, normal.y * y_scale);

            var out: VertexOutput;
            out.color = instance.color;

            if (model.position.x == -0.5 && model.position.y == -0.5) {
                out.position = camera.view_proj * vec4<f32>(instance.pos1, 0.0, 1.0) + vec4<f32>(scaled_normal, 0.0, 0.0);
            } elseif (model.position.x == 0.5 && model.position.y == -0.5) {
                out.position = camera.view_proj * vec4<f32>(instance.pos1, 0.0, 1.0) - vec4<f32>(scaled_normal, 0.0, 0.0);
            } elseif (model.position.x == 0.5 && model.position.y == 0.5) {
                out.position = camera.view_proj * vec4<f32>(instance.pos2, 0.0, 1.0) - vec4<f32>(scaled_normal, 0.0, 0.0);
            } elseif (model.position.x == -0.5 && model.position.y == 0.5) {
                out.position = camera.view_proj * vec4<f32>(instance.pos2, 0.0, 1.0) + vec4<f32>(scaled_normal, 0.0, 0.0);
            } else {
                out.position = camera.view_proj * vec4<f32>(model.position, 1.0);
            }

            return out;
        }

        [[stage(fragment)]]
        fn main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
            return in.color;
        }"
    .to_string();

    let resource = graphic_service.create_shader_resource(
        "Shaders/Draw Line",
        code,
        ShaderResourceSettings {
            binding_groups: vec![
                ShaderBindingGroup {
                    bindings: vec![ShaderBinding {
                        visibility: ShaderBindingVisibility::Vertex,
                        ty: ShaderBindingType::Uniform,
                    }],
                },
                ShaderBindingGroup {
                    bindings: vec![ShaderBinding {
                        visibility: ShaderBindingVisibility::Vertex,
                        ty: ShaderBindingType::Uniform,
                    }],
                },
            ],
            instance_attributes: vec![
                ShaderInstanceAttribute {
                    location: 5,
                    ty: ShaderInstanceAttributeType::Vector2d,
                },
                ShaderInstanceAttribute {
                    location: 6,
                    ty: ShaderInstanceAttributeType::Vector2d,
                },
                ShaderInstanceAttribute {
                    location: 7,
                    ty: ShaderInstanceAttributeType::Uint,
                },
                ShaderInstanceAttribute {
                    location: 8,
                    ty: ShaderInstanceAttributeType::Vector4d,
                },
            ],
        },
    )?;

    resource_container.add("Shaders/Draw Line", resource);

    Ok(())
}

pub fn load_draw_line_material(resource_container: ResourceContainer) -> FruityResult<()> {
    let graphic_service = resource_container.require::<dyn GraphicService>();
    let graphic_service = graphic_service.read();

    let shader = resource_container.get::<dyn ShaderResource>("Shaders/Draw Line");

    let resource = graphic_service.create_material_resource(
        "Materials/Draw Line",
        MaterialResourceSettings {
            shader,
            bindings: vec![
                MaterialSettingsBinding::Camera { bind_group: 0 },
                MaterialSettingsBinding::RenderSurfaceSize { bind_group: 1 },
            ],
            instance_attributes: hashmap! {
                "pos1".to_string() => MaterialSettingsInstanceAttribute::Vector2d {
                    location: 5,
                },
                "pos2".to_string() => MaterialSettingsInstanceAttribute::Vector2d {
                    location: 6,
                },
                "width".to_string() => MaterialSettingsInstanceAttribute::Uint {
                    location: 7,
                },
                "color".to_string() => MaterialSettingsInstanceAttribute::Vector4d {
                    location: 8,
                },
            },
        },
    )?;

    resource_container.add("Materials/Draw Line", resource);

    Ok(())
}

pub fn load_draw_dotted_line_shader(resource_container: ResourceContainer) -> FruityResult<()> {
    let graphic_service = resource_container.require::<dyn GraphicService>();
    let graphic_service = graphic_service.read();

    let code = "
        [[block]]
        struct CameraUniform {
            view_proj: mat4x4<f32>;
        };
        
        [[block]]
        struct RenderSurfaceSizeUniform {
            value: vec2<f32>;
        };
        
        struct VertexInput {
            [[location(0)]] position: vec3<f32>;
            [[location(1)]] tex_coords: vec2<f32>;
            [[location(2)]] normal: vec3<f32>;
        };
        
        struct InstanceInput {
            [[location(5)]] transform_0: vec4<f32>;
            [[location(6)]] transform_1: vec4<f32>;
            [[location(7)]] transform_2: vec4<f32>;
            [[location(8)]] transform_3: vec4<f32>;
            [[location(9)]] pos1: vec2<f32>;
            [[location(10)]] pos2: vec2<f32>;
            [[location(11)]] width: u32;
            [[location(12)]] color: vec4<f32>;
        };
        
        struct VertexOutput {
            [[builtin(position)]] position: vec4<f32>;
            [[location(0)]] color: vec4<f32>;
            [[location(1)]] tex_coords: vec2<f32>;
            [[location(2)]] xwidth: f32;
            [[location(3)]] ywidth: f32;
        };

        [[group(0), binding(0)]]
        var<uniform> camera: CameraUniform;

        [[group(1), binding(0)]]
        var<uniform> render_surface_size: RenderSurfaceSizeUniform;

        [[stage(vertex)]]
        fn main(
            model: VertexInput,
            instance: InstanceInput,
        ) -> VertexOutput {
            var out: VertexOutput;

            let transform = mat4x4<f32>(
                instance.transform_0,
                instance.transform_1,
                instance.transform_2,
                instance.transform_3,
            );
        
            var diff = camera.view_proj * transform * (vec4<f32>(instance.pos2, 0.0, 1.0) - vec4<f32>(instance.pos1, 0.0, 0.0));
            let x_scale = f32(instance.width) / render_surface_size.value.x;
            let y_scale = f32(instance.width) / render_surface_size.value.y;
            let normal = normalize(vec2<f32>(-diff.y, diff.x));
            let scaled_normal = vec2<f32>(normal.x * x_scale, normal.y * y_scale);

            out.color = instance.color;
            out.tex_coords = model.tex_coords;
            out.xwidth = f32(instance.width) / length(diff) / render_surface_size.value.x;

            if (model.position.x == -0.5 && model.position.y == -0.5) {
                out.position = camera.view_proj * transform * vec4<f32>(instance.pos1, 0.0, 1.0) + vec4<f32>(scaled_normal, 0.0, 0.0);
            } elseif (model.position.x == 0.5 && model.position.y == -0.5) {
                out.position = camera.view_proj * transform * vec4<f32>(instance.pos1, 0.0, 1.0) - vec4<f32>(scaled_normal, 0.0, 0.0);
            } elseif (model.position.x == 0.5 && model.position.y == 0.5) {
                out.position = camera.view_proj * transform * vec4<f32>(instance.pos2, 0.0, 1.0) - vec4<f32>(scaled_normal, 0.0, 0.0);
            } elseif (model.position.x == -0.5 && model.position.y == 0.5) {
                out.position = camera.view_proj * transform * vec4<f32>(instance.pos2, 0.0, 1.0) + vec4<f32>(scaled_normal, 0.0, 0.0);
            } else {
                out.position = camera.view_proj * transform * vec4<f32>(model.position, 1.0);
            }

            return out;
        }

        [[stage(fragment)]]
        fn main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
            let dotx = i32(floor(in.tex_coords.y / in.xwidth / 25.0));

            if(dotx % 2 == 0) {
                return in.color;
            } else {
                return vec4<f32>(0.0);
            }
        }"
    .to_string();

    let resource = graphic_service.create_shader_resource(
        "Shaders/Draw Dotted Line",
        code,
        ShaderResourceSettings {
            binding_groups: vec![
                ShaderBindingGroup {
                    bindings: vec![ShaderBinding {
                        visibility: ShaderBindingVisibility::Vertex,
                        ty: ShaderBindingType::Uniform,
                    }],
                },
                ShaderBindingGroup {
                    bindings: vec![ShaderBinding {
                        visibility: ShaderBindingVisibility::Vertex,
                        ty: ShaderBindingType::Uniform,
                    }],
                },
            ],
            instance_attributes: vec![
                ShaderInstanceAttribute {
                    location: 5,
                    ty: ShaderInstanceAttributeType::Vector4d,
                },
                ShaderInstanceAttribute {
                    location: 6,
                    ty: ShaderInstanceAttributeType::Vector4d,
                },
                ShaderInstanceAttribute {
                    location: 7,
                    ty: ShaderInstanceAttributeType::Vector4d,
                },
                ShaderInstanceAttribute {
                    location: 8,
                    ty: ShaderInstanceAttributeType::Vector4d,
                },
                ShaderInstanceAttribute {
                    location: 9,
                    ty: ShaderInstanceAttributeType::Vector2d,
                },
                ShaderInstanceAttribute {
                    location: 10,
                    ty: ShaderInstanceAttributeType::Vector2d,
                },
                ShaderInstanceAttribute {
                    location: 11,
                    ty: ShaderInstanceAttributeType::Uint,
                },
                ShaderInstanceAttribute {
                    location: 12,
                    ty: ShaderInstanceAttributeType::Vector4d,
                },
            ],
        },
    )?;

    resource_container.add("Shaders/Draw Dotted Line", resource);

    Ok(())
}

pub fn load_draw_dotted_line_material(resource_container: ResourceContainer) -> FruityResult<()> {
    let graphic_service = resource_container.require::<dyn GraphicService>();
    let graphic_service = graphic_service.read();

    let shader = resource_container.get::<dyn ShaderResource>("Shaders/Draw Dotted Line");

    let resource = graphic_service.create_material_resource(
        "Materials/Draw Dotted Line",
        MaterialResourceSettings {
            shader,
            bindings: vec![
                MaterialSettingsBinding::Camera { bind_group: 0 },
                MaterialSettingsBinding::RenderSurfaceSize { bind_group: 1 },
            ],
            instance_attributes: hashmap! {
                "transform".to_string() => MaterialSettingsInstanceAttribute::Matrix4 {
                    vec0_location: 5,
                    vec1_location: 6,
                    vec2_location: 7,
                    vec3_location: 8,
                },
                "pos1".to_string() => MaterialSettingsInstanceAttribute::Vector2d {
                    location: 9,
                },
                "pos2".to_string() => MaterialSettingsInstanceAttribute::Vector2d {
                    location: 10,
                },
                "width".to_string() => MaterialSettingsInstanceAttribute::Uint {
                    location: 11,
                },
                "color".to_string() => MaterialSettingsInstanceAttribute::Vector4d {
                    location: 12,
                },
            },
        },
    )?;

    resource_container.add("Materials/Draw Dotted Line", resource);

    Ok(())
}

pub fn load_draw_rect_shader(resource_container: ResourceContainer) -> FruityResult<()> {
    let graphic_service = resource_container.require::<dyn GraphicService>();
    let graphic_service = graphic_service.read();

    let code = "
        [[block]]
        struct CameraUniform {
            view_proj: mat4x4<f32>;
        };
        
        [[block]]
        struct RenderSurfaceSizeUniform {
            value: vec2<f32>;
        };
        
        [[block]]
        struct ViewportSizeUniform {
            value: vec2<f32>;
        };
        
        struct VertexInput {
            [[location(0)]] position: vec3<f32>;
            [[location(1)]] tex_coords: vec2<f32>;
            [[location(2)]] normal: vec3<f32>;
        };
        
        struct InstanceInput {
            [[location(5)]] transform_0: vec4<f32>;
            [[location(6)]] transform_1: vec4<f32>;
            [[location(7)]] transform_2: vec4<f32>;
            [[location(8)]] transform_3: vec4<f32>;
            [[location(9)]] bottom_left: vec2<f32>;
            [[location(10)]] top_right: vec2<f32>;
            [[location(11)]] width: u32;
            [[location(12)]] fill_color: vec4<f32>;
            [[location(13)]] border_color: vec4<f32>;
        };
        
        struct VertexOutput {
            [[builtin(position)]] position: vec4<f32>;
            [[location(0)]] border_color: vec4<f32>;
            [[location(1)]] fill_color: vec4<f32>;
            [[location(2)]] tex_coords: vec2<f32>;
            [[location(3)]] xwidth: f32;
            [[location(4)]] ywidth: f32;
        };

        [[group(0), binding(0)]]
        var<uniform> camera: CameraUniform;

        [[group(1), binding(0)]]
        var<uniform> render_surface_size: RenderSurfaceSizeUniform;

        [[stage(vertex)]]
        fn main(
            model: VertexInput,
            instance: InstanceInput,
        ) -> VertexOutput {
            var out: VertexOutput;

            let transform = mat4x4<f32>(
                instance.transform_0,
                instance.transform_1,
                instance.transform_2,
                instance.transform_3,
            );

            let scale = vec2<f32>(
                sqrt(pow(transform[0][0], 2.0) + pow(transform[0][1], 2.0)),
                sqrt(pow(transform[1][0], 2.0) + pow(transform[1][1], 2.0)),
            );

            let scale_transform = mat4x4<f32>(
                vec4<f32>(scale[0], 0.0, 0.0, 0.0),
                vec4<f32>(0.0, scale[1], 0.0, 0.0),
                vec4<f32>(0.0, 0.0, 1.0, 0.0),
                vec4<f32>(0.0, 0.0, 0.0, 1.0),
            );
        
            var diff = camera.view_proj * scale_transform * (vec4<f32>(instance.top_right, 0.0, 1.0) - vec4<f32>(instance.bottom_left, 0.0, 0.0));

            out.fill_color = instance.fill_color;
            out.border_color = instance.border_color;
            out.tex_coords = model.tex_coords;
            out.xwidth = f32(instance.width) / diff.x / render_surface_size.value.x;
            out.ywidth = f32(instance.width) / diff.y / render_surface_size.value.y;

            if (model.position.x == -0.5 && model.position.y == -0.5) {
                out.position = camera.view_proj * transform * vec4<f32>(instance.bottom_left.x, instance.bottom_left.y, 0.0, 1.0);
            } elseif (model.position.x == 0.5 && model.position.y == -0.5) {
                out.position = camera.view_proj * transform * vec4<f32>(instance.top_right.x, instance.bottom_left.y, 0.0, 1.0);
            } elseif (model.position.x == 0.5 && model.position.y == 0.5) {
                out.position = camera.view_proj * transform * vec4<f32>(instance.top_right.x, instance.top_right.y, 0.0, 1.0);
            } elseif (model.position.x == -0.5 && model.position.y == 0.5) {
                out.position = camera.view_proj * transform * vec4<f32>(instance.bottom_left.x, instance.top_right.y, 0.0, 1.0);
            } else {
                out.position = camera.view_proj * transform * vec4<f32>(model.position, 1.0);
            }

            return out;
        }

        [[stage(fragment)]]
        fn main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
            if(
                in.tex_coords.x < in.xwidth ||
                in.tex_coords.x > (1.0 - in.xwidth) ||
                in.tex_coords.y < in.ywidth ||
                in.tex_coords.y > (1.0 - in.ywidth)
            ) {
                return in.border_color;
            } else {
                return in.fill_color;
            }
        }"
    .to_string();

    let resource = graphic_service.create_shader_resource(
        "Shaders/Draw Rect",
        code,
        ShaderResourceSettings {
            binding_groups: vec![
                ShaderBindingGroup {
                    bindings: vec![ShaderBinding {
                        visibility: ShaderBindingVisibility::Vertex,
                        ty: ShaderBindingType::Uniform,
                    }],
                },
                ShaderBindingGroup {
                    bindings: vec![ShaderBinding {
                        visibility: ShaderBindingVisibility::Vertex,
                        ty: ShaderBindingType::Uniform,
                    }],
                },
            ],
            instance_attributes: vec![
                ShaderInstanceAttribute {
                    location: 5,
                    ty: ShaderInstanceAttributeType::Vector4d,
                },
                ShaderInstanceAttribute {
                    location: 6,
                    ty: ShaderInstanceAttributeType::Vector4d,
                },
                ShaderInstanceAttribute {
                    location: 7,
                    ty: ShaderInstanceAttributeType::Vector4d,
                },
                ShaderInstanceAttribute {
                    location: 8,
                    ty: ShaderInstanceAttributeType::Vector4d,
                },
                ShaderInstanceAttribute {
                    location: 9,
                    ty: ShaderInstanceAttributeType::Vector2d,
                },
                ShaderInstanceAttribute {
                    location: 10,
                    ty: ShaderInstanceAttributeType::Vector2d,
                },
                ShaderInstanceAttribute {
                    location: 11,
                    ty: ShaderInstanceAttributeType::Uint,
                },
                ShaderInstanceAttribute {
                    location: 12,
                    ty: ShaderInstanceAttributeType::Vector4d,
                },
                ShaderInstanceAttribute {
                    location: 13,
                    ty: ShaderInstanceAttributeType::Vector4d,
                },
            ],
        },
    )?;

    resource_container.add("Shaders/Draw Rect", resource);

    Ok(())
}

pub fn load_draw_rect_material(resource_container: ResourceContainer) -> FruityResult<()> {
    let graphic_service = resource_container.require::<dyn GraphicService>();
    let graphic_service = graphic_service.read();

    let shader = resource_container.get::<dyn ShaderResource>("Shaders/Draw Rect");

    let resource = graphic_service.create_material_resource(
        "Materials/Draw Rect",
        MaterialResourceSettings {
            shader,
            bindings: vec![
                MaterialSettingsBinding::Camera { bind_group: 0 },
                MaterialSettingsBinding::RenderSurfaceSize { bind_group: 1 },
            ],
            instance_attributes: hashmap! {
                "transform".to_string() => MaterialSettingsInstanceAttribute::Matrix4 {
                    vec0_location: 5,
                    vec1_location: 6,
                    vec2_location: 7,
                    vec3_location: 8,
                },
                "bottom_left".to_string() => MaterialSettingsInstanceAttribute::Vector2d {
                    location: 9,
                },
                "top_right".to_string() => MaterialSettingsInstanceAttribute::Vector2d {
                    location: 10,
                },
                "width".to_string() => MaterialSettingsInstanceAttribute::Uint {
                    location: 11,
                },
                "fill_color".to_string() => MaterialSettingsInstanceAttribute::Vector4d {
                    location: 12,
                },
                "border_color".to_string() => MaterialSettingsInstanceAttribute::Vector4d {
                    location: 13,
                },
            },
        },
    )?;

    resource_container.add("Materials/Draw Rect", resource);

    Ok(())
}

pub fn load_draw_arc_shader(resource_container: ResourceContainer) -> FruityResult<()> {
    let graphic_service = resource_container.require::<dyn GraphicService>();
    let graphic_service = graphic_service.read();

    let code = "
        [[block]]
        struct CameraUniform {
            view_proj: mat4x4<f32>;
        };
        
        [[block]]
        struct RenderSurfaceSizeUniform {
            value: vec2<f32>;
        };
        
        struct VertexInput {
            [[location(0)]] position: vec3<f32>;
            [[location(1)]] tex_coords: vec2<f32>;
            [[location(2)]] normal: vec3<f32>;
        };
        
        struct InstanceInput {
            [[location(5)]] transform_0: vec4<f32>;
            [[location(6)]] transform_1: vec4<f32>;
            [[location(7)]] transform_2: vec4<f32>;
            [[location(8)]] transform_3: vec4<f32>;
            [[location(9)]] center: vec2<f32>;
            [[location(10)]] radius: f32;
            [[location(11)]] fill_color: vec4<f32>;
            [[location(12)]] border_color: vec4<f32>;
            [[location(13)]] width: u32;
            [[location(14)]] angle_start: f32;
            [[location(15)]] angle_end: f32;
        };
        
        struct VertexOutput {
            [[builtin(position)]] position: vec4<f32>;
            [[location(0)]] border_color: vec4<f32>;
            [[location(1)]] fill_color: vec4<f32>;
            [[location(2)]] tex_coords: vec2<f32>;
            [[location(3)]] xwidth: f32;
            [[location(4)]] ywidth: f32;
            [[location(5)]] angle_start: f32;
            [[location(6)]] angle_end: f32;
        };

        [[group(0), binding(0)]]
        var<uniform> camera: CameraUniform;

        [[group(1), binding(0)]]
        var<uniform> render_surface_size: RenderSurfaceSizeUniform;

        [[stage(vertex)]]
        fn main(
            model: VertexInput,
            instance: InstanceInput,
        ) -> VertexOutput {
            let transform = mat4x4<f32>(
                instance.transform_0,
                instance.transform_1,
                instance.transform_2,
                instance.transform_3,
            );
        
            var bottom_left = instance.center - vec2<f32>(instance.radius, instance.radius);
            var top_right = instance.center + vec2<f32>(instance.radius, instance.radius);
            var diff = camera.view_proj * transform * (vec4<f32>(top_right, 0.0, 1.0) - vec4<f32>(bottom_left, 0.0, 0.0));

            var out: VertexOutput;
            out.fill_color = instance.fill_color;
            out.border_color = instance.border_color;
            out.tex_coords = model.tex_coords;
            out.xwidth = f32(instance.width) / diff.x / render_surface_size.value.x;
            out.ywidth = f32(instance.width) / diff.y / render_surface_size.value.y;
            out.angle_start = instance.angle_start;
            out.angle_end = instance.angle_end;

            if (model.position.x == -0.5 && model.position.y == -0.5) {
                out.position = camera.view_proj * transform * vec4<f32>(bottom_left.x, bottom_left.y, 0.0, 1.0);
            } elseif (model.position.x == 0.5 && model.position.y == -0.5) {
                out.position = camera.view_proj * transform * vec4<f32>(top_right.x, bottom_left.y, 0.0, 1.0);
            } elseif (model.position.x == 0.5 && model.position.y == 0.5) {
                out.position = camera.view_proj * transform * vec4<f32>(top_right.x, top_right.y, 0.0, 1.0);
            } elseif (model.position.x == -0.5 && model.position.y == 0.5) {
                out.position = camera.view_proj * transform * vec4<f32>(bottom_left.x, top_right.y, 0.0, 1.0);
            } else {
                out.position = camera.view_proj * transform * vec4<f32>(model.position, 1.0);
            }

            return out;
        }

        [[stage(fragment)]]
        fn main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
            let circle_coords = 2.0 * (in.tex_coords - vec2<f32>(0.5, 0.5));
            let angle = atan2(-circle_coords.y, circle_coords.x);
            let border_radius = 1.0 - (abs(circle_coords.x) * in.xwidth + abs(circle_coords.y) * in.ywidth);

            if(
                length(circle_coords) <= 1.0 &&
                angle <= in.angle_end &&
                angle >= in.angle_start
            ) {
                if(length(circle_coords) <= border_radius) {
                    return in.fill_color;
                } else {
                    return in.border_color;
                }
            } else {
                return vec4<f32>(0.0, 0.0, 0.0, 0.0);
            }
        }"
    .to_string();

    let resource = graphic_service.create_shader_resource(
        "Shaders/Draw Arc",
        code,
        ShaderResourceSettings {
            binding_groups: vec![
                ShaderBindingGroup {
                    bindings: vec![ShaderBinding {
                        visibility: ShaderBindingVisibility::Vertex,
                        ty: ShaderBindingType::Uniform,
                    }],
                },
                ShaderBindingGroup {
                    bindings: vec![ShaderBinding {
                        visibility: ShaderBindingVisibility::Vertex,
                        ty: ShaderBindingType::Uniform,
                    }],
                },
            ],
            instance_attributes: vec![
                ShaderInstanceAttribute {
                    location: 5,
                    ty: ShaderInstanceAttributeType::Vector4d,
                },
                ShaderInstanceAttribute {
                    location: 6,
                    ty: ShaderInstanceAttributeType::Vector4d,
                },
                ShaderInstanceAttribute {
                    location: 7,
                    ty: ShaderInstanceAttributeType::Vector4d,
                },
                ShaderInstanceAttribute {
                    location: 8,
                    ty: ShaderInstanceAttributeType::Vector4d,
                },
                ShaderInstanceAttribute {
                    location: 9,
                    ty: ShaderInstanceAttributeType::Vector2d,
                },
                ShaderInstanceAttribute {
                    location: 10,
                    ty: ShaderInstanceAttributeType::Float,
                },
                ShaderInstanceAttribute {
                    location: 11,
                    ty: ShaderInstanceAttributeType::Vector4d,
                },
                ShaderInstanceAttribute {
                    location: 12,
                    ty: ShaderInstanceAttributeType::Vector4d,
                },
                ShaderInstanceAttribute {
                    location: 13,
                    ty: ShaderInstanceAttributeType::Uint,
                },
                ShaderInstanceAttribute {
                    location: 14,
                    ty: ShaderInstanceAttributeType::Float,
                },
                ShaderInstanceAttribute {
                    location: 15,
                    ty: ShaderInstanceAttributeType::Float,
                },
            ],
        },
    )?;

    resource_container.add("Shaders/Draw Arc", resource);

    Ok(())
}

pub fn load_draw_arc_material(resource_container: ResourceContainer) -> FruityResult<()> {
    let graphic_service = resource_container.require::<dyn GraphicService>();
    let graphic_service = graphic_service.read();

    let shader = resource_container.get::<dyn ShaderResource>("Shaders/Draw Arc");

    let resource = graphic_service.create_material_resource(
        "Materials/Draw Arc",
        MaterialResourceSettings {
            shader,
            bindings: vec![
                MaterialSettingsBinding::Camera { bind_group: 0 },
                MaterialSettingsBinding::RenderSurfaceSize { bind_group: 1 },
            ],
            instance_attributes: hashmap! {
                "transform".to_string() => MaterialSettingsInstanceAttribute::Matrix4 {
                    vec0_location: 5,
                    vec1_location: 6,
                    vec2_location: 7,
                    vec3_location: 8,
                },
                "center".to_string() => MaterialSettingsInstanceAttribute::Vector2d {
                    location: 9,
                },
                "radius".to_string() => MaterialSettingsInstanceAttribute::Float {
                    location: 10,
                },
                "fill_color".to_string() => MaterialSettingsInstanceAttribute::Vector4d {
                    location: 11,
                },
                "border_color".to_string() => MaterialSettingsInstanceAttribute::Vector4d {
                    location: 12,
                },
                "width".to_string() => MaterialSettingsInstanceAttribute::Uint {
                    location: 13,
                },
                "angle_start".to_string() => MaterialSettingsInstanceAttribute::Float {
                    location: 14,
                },
                "angle_end".to_string() => MaterialSettingsInstanceAttribute::Float {
                    location: 15,
                },
            },
        },
    )?;

    resource_container.add("Materials/Draw Arc", resource);

    Ok(())
}
