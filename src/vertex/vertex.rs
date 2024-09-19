use bytemuck;

/// A vertex is a 3D point in space with a color.
///
/// The color is represented as an RGB value, with each component being a
/// `f32` between 0.0 and 1.0.
///
/// The position is represented as a 3D vector, with each component being a
/// `f32` representing the x, y and z coordinates respectively.
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    /// The position of the vertex in 3D space.
    pub position: [f32; 3],
    /// The color of the vertex.
    pub color: [f32; 3],
}

impl Vertex {
    /// Returns the vertex buffer layout for the `Vertex` type.
    ///
    /// The layout is suitable for use with a vertex shader that takes a
    /// `vec3<f32>` for the position and a `vec3<f32>` for the color.
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}
