struct VertexOutput{
    @builtin(position) clip_position: vec4<f32>
}

// Entry Point
// Vertex Shader
@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32
) -> VertexOutput {
    // let = const | var = let + needs specified type
    var out: VertexOutput;
    let x = f32(1 - i32(in_vertex_index)) * 0.5;
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    return out;
}

// Fragmnt Shader
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32>{
    return vec4<f32>(0.3, 0.2, 0.1, 1.0);
}

