struct Vertex {
    @location(0) colour: vec4<f32>,
    @location(1) position: vec2<f32>,
};

struct VertexShaderOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) colour: vec4<f32>,
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
    output.colour = vertex.colour;

    return output;
}

@fragment
fn fs_main(in: VertexShaderOutput) -> @location(0) vec4<f32> {
    return in.colour;
    // return vec4<f32>(1.0, 0.5, 0.2, 1.0);
}