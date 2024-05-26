struct VertexInput{
   @location(0) x: f32,
   @location(1) y: f32,
   @location(2) tx: f32,
   @location(3) ty: f32
}

@vertex
fn vs_main(inp: VertexInput) -> FragDetails{
    var frag_details: FragDetails;
    frag_details.pos = vec4(inp.x, inp.y, 1.0, 1.0);
    frag_details.tex_coords = vec2(inp.tx, inp.ty);
    return frag_details;
}

struct FragDetails{
    @builtin(position) pos: vec4<f32>,
    @location(0) tex_coords: vec2<f32>
}

@group(0) @binding(0)
var text: texture_2d<f32>;
@group(0) @binding(1)
var samp: sampler;

@fragment
fn fs_main(det: FragDetails) -> @location(0) vec4<f32> {
    return textureSample(text, samp, det.tex_coords);
}
