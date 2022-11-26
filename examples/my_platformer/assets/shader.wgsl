// Vertex shader

[[block]]
struct CameraUniform {
    view_proj: mat4x4<f32>;
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
    // [[location(9)]] tex_area_top_left: vec2<f32>;
    // [[location(10)]] tex_area_bottom_right: vec2<f32>;
};

struct VertexOutput {
    [[builtin(position)]] position: vec4<f32>;
    [[location(0)]] tex_coords: vec2<f32>;
};

[[group(1), binding(0)]]
var<uniform> camera: CameraUniform;

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

    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.position = camera.view_proj * transform * vec4<f32>(model.position, 1.0);
    return out;
}

// Fragment shader

[[group(0), binding(0)]]
var t_diffuse: texture_2d<f32>;
[[group(0), binding(1)]]
var s_diffuse: sampler;

[[stage(fragment)]]
fn main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}