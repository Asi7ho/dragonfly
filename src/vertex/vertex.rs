use bytemuck;
use std::rc::Rc;

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

#[derive(Debug, Default)]
pub enum Figure {
    #[default]
    Triangle,
    Pentagon,
    Rectange,
    Trapezoid,
    Parallelogram,
    Circle,
}

impl Figure {
    /// Returns the vertices and indices for the given figure.
    pub fn get_vertices_and_indices(&self) -> (Rc<[Vertex]>, Rc<[u16]>) {
        match self {
            Figure::Triangle => (
                Rc::from(super::TRIANGLE_VERTICES),
                Rc::from(super::TRIANGLE_INDICES),
            ),
            Figure::Pentagon => (
                Rc::from(super::PENTAGON_VERTICES),
                Rc::from(super::PENTAGON_INDICES),
            ),
            Figure::Rectange => (
                Rc::from(super::RECTANGLE_VERTICES),
                Rc::from(super::RECTANGLE_INDICES),
            ),
            Figure::Trapezoid => (
                Rc::from(super::TRAPEZOID_VERTICES),
                Rc::from(super::TRAPEZOID_INDICES),
            ),
            Figure::Parallelogram => (
                Rc::from(super::PARALLELOGRAM_VERTICES),
                Rc::from(super::PARALLELOGRAM_INDICES),
            ),
            Figure::Circle => (
                Rc::from(crate::circle_vertices!(64).into_boxed_slice()),
                Rc::from(crate::circle_indices!(64).into_boxed_slice()),
            ),
        }
    }

    /// Returns the figure at the given index.
    ///
    /// If the index is not in the range 0..4, the default figure (Triangle) is
    /// returned.
    pub fn get_figure(i: u8) -> Self {
        match i {
            0 => Figure::Triangle,
            1 => Figure::Pentagon,
            2 => Figure::Rectange,
            3 => Figure::Trapezoid,
            4 => Figure::Parallelogram,
            5 => Figure::Circle,
            _ => Figure::Triangle,
        }
    }
}
