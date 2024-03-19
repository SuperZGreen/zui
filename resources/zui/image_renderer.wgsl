struct Vertex {
    @location(0) position: vec2<f32>,
    @location(1) uv: vec2<f32>,
};

struct VertexShaderOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(
    vertex: Vertex,
) -> VertexShaderOutput {

    var output: VertexShaderOutput;

    output.clip_position = vec4<f32>(
        vertex.position.xy,
        0.0,
        1.0
    );
    output.uv = vertex.uv;

    return output;
}

@group(0) @binding(0)
var texture: texture_2d<f32>;

@group(0) @binding(1)
var texture_sampler: sampler;

@fragment
fn fs_main(in: VertexShaderOutput) -> @location(0) vec4<f32> {
    let texture_colour = textureSample(texture, texture_sampler, in.uv);
    return texture_colour;
}
