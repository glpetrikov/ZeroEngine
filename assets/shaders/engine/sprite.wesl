@group(0) @binding(0) var sprite_texture: texture_2d<f32>;
@group(0) @binding(1) var sprite_sampler: sampler;

struct Material {
    tint: vec4<f32>,
    params: vec4<f32>,
}

@group(0) @binding(2) var<uniform> material: Material;

@group(1) @binding(0) var<uniform> transform: mat4x4<f32>;
@group(2) @binding(0) var<uniform> view_projection: mat4x4<f32>;

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) tex_coord: vec2<f32>,
}

@vertex
fn vs_main(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    out.position = view_projection * transform * vec4<f32>(vertex.position, 1.0);
    out.color = vertex.color;
    out.tex_coord = vec2<f32>(vertex.position.x + 0.5, -vertex.position.y + 0.5);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let mode = material.params.x;
    let strength = material.params.y;
    let saturation_threshold = material.params.z;

    let sampled = textureSample(sprite_texture, sprite_sampler, in.tex_coord) * in.color;

    if mode < 0.5 {
        return sampled;
    }

    if mode < 1.5 {
        return sampled * material.tint;
    }

    let max_channel = max(sampled.r, max(sampled.g, sampled.b));
    let min_channel = min(sampled.r, min(sampled.g, sampled.b));
    let saturation = max_channel - min_channel;
    let grayscale_factor = 1.0 - smoothstep(0.0, saturation_threshold, saturation);
    let gray = dot(sampled.rgb, vec3<f32>(0.299, 0.587, 0.114));
    let tinted_gray = gray * material.tint.rgb;
    let mix_factor = clamp(grayscale_factor * strength, 0.0, 1.0);

    return vec4<f32>(mix(sampled.rgb, tinted_gray, mix_factor), sampled.a * material.tint.a);
}
