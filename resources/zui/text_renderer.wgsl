struct Vertex {
    @location(0) colour: vec4<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) position: vec2<f32>,
    @location(3) clip_bounds: vec4<f32>,
};

struct VertexShaderOutput {
    // Note that position goes from (0,0) in top left, to 
    @builtin(position) clip_position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) clip_bounds: vec4<f32>,
    @location(2) uv: vec2<f32>,
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
    output.colour = vertex.colour;
    output.clip_bounds = vertex.clip_bounds;

    return output;
}

@group(0) @binding(0)
var texture: texture_2d<f32>;

@group(0) @binding(1)
var texture_sampler: sampler;


@fragment
fn fs_main(in: VertexShaderOutput) -> @location(0) vec4<f32> {
    let coverage = textureSample(texture, texture_sampler, in.uv).r;

    // let clip_x_min = in.clip_bounds[0];
    // let clip_x_max = in.clip_bounds[1];
    // let clip_y_min = in.clip_bounds[2];
    // let clip_y_max = in.clip_bounds[3];

    // if in.clip_position.x < clip_x_min
    //     || in.clip_position.x > clip_x_max
    //     || in.clip_position.y < clip_y_min
    //     || in.clip_position.y > clip_y_max
    // {
    //     discard;
    // }

    let colour = vec4<f32>(in.colour.rgb, coverage);

    // let colour = vec4<f32>(1f, 1f, 1f, 1f);

    return colour;
}