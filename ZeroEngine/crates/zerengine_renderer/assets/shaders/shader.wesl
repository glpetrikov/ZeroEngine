@group(0) @binding(0) var texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;

@group(1) @binding(0) var<uniform> model: mat4x4<f32>;

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) tex_coord: vec2<f32>,
};

@vertex
fn vs_main(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    out.position = model * vec4<f32>(vertex.position, 1.0);
    out.color = vertex.color;
    out.tex_coord = vec2<f32>(0.5 * (vertex.position.x + 1), 0.5 * (-vertex.position.y + 1));
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color) * textureSample(texture, texture_sampler, in.tex_coord);
}