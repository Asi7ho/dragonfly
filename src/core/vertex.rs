use bytemuck;
use std::rc::Rc;

/// Generates a vector of vertices for a circle with the given number of
/// segments.
///
/// The circle is centered at the origin and has a radius of 0.5. The vertices
/// are arranged in a counter-clockwise direction.
///
/// # Arguments
///
/// * `num_segments` - The number of segments to divide the circle into.
///
/// # Returns
///
/// A vector of `Vertex` structs representing the vertices of the circle.
macro_rules! circle_vertices {
    ($num_segments:expr) => {{
        const NUM_SEGMENTS: usize = $num_segments;
        const TWO_PI: f32 = std::f32::consts::PI * 2.0;

        let vertices: Vec<Vertex> = std::iter::once(Vertex {
            position: [0.0, 0.0, 0.0],
            color: [0.5, 0.5, 0.5],
        })
        .chain((0..(NUM_SEGMENTS + 1)).map(|i| {
            let angle = i as f32 * TWO_PI / NUM_SEGMENTS as f32;
            Vertex {
                position: [0.5 * angle.cos(), 0.5 * angle.sin(), 0.0],
                color: [
                    angle.sin(),
                    (angle + 2.0 * TWO_PI / 6.0).sin(),
                    (angle + 4.0 * TWO_PI / 6.0).sin(),
                ],
            }
        }))
        .collect();

        vertices
    }};
}

/// Generates a vector of indices for a circle with the given number of
/// segments.
///
/// The circle is assumed to have `num_segments + 1` vertices, with the first
/// vertex being the center of the circle. The indices are arranged in a
/// counter-clockwise direction, starting at the second vertex and ending at the
/// second-to-last vertex.
///
/// # Arguments
///
/// * `num_segments` - The number of segments to divide the circle into.
///
/// # Returns
///
/// A vector of `u16` values representing the indices of the vertices that make
/// up the triangles that form the circle.
macro_rules! circle_indices {
    ($num_segments:expr) => {{
        const NUM_SEGMENTS: usize = $num_segments;

        let indices: Vec<u16> = (1..(NUM_SEGMENTS + 1) as u16)
            .flat_map(|i| [0, i, i + 1])
            .collect();

        indices
    }};
}

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
    position: [f32; 3],
    /// The color of the vertex.
    color: [f32; 3],
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
            Figure::Triangle => (Rc::from(TRIANGLE_VERTICES), Rc::from(TRIANGLE_INDICES)),
            Figure::Pentagon => (Rc::from(PENTAGON_VERTICES), Rc::from(PENTAGON_INDICES)),
            Figure::Rectange => (Rc::from(RECTANGLE_VERTICES), Rc::from(RECTANGLE_INDICES)),
            Figure::Trapezoid => (Rc::from(TRAPEZOID_VERTICES), Rc::from(TRAPEZOID_INDICES)),
            Figure::Parallelogram => (
                Rc::from(PARALLELOGRAM_VERTICES),
                Rc::from(PARALLELOGRAM_INDICES),
            ),
            Figure::Circle => (
                Rc::from(circle_vertices!(64).into_boxed_slice()),
                Rc::from(circle_indices!(64).into_boxed_slice()),
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

// Triangle
const TRIANGLE_VERTICES: &[Vertex] = &[
    Vertex {
        position: [0.0, 0.5, 0.0],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [-0.5, -0.5, 0.0],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [0.5, -0.5, 0.0],
        color: [0.0, 0.0, 1.0],
    },
];
const TRIANGLE_INDICES: &[u16] = &[0, 1, 2];

// Hexagon
const PENTAGON_VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.0868241, 0.49240386, 0.0],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [-0.49513406, 0.06958647, 0.0],
        color: [0.5, 0.5, 0.0],
    },
    Vertex {
        position: [-0.21918549, -0.44939706, 0.0],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [0.35966998, -0.3473291, 0.0],
        color: [0.0, 0.5, 0.5],
    },
    Vertex {
        position: [0.44147372, 0.2347359, 0.0],
        color: [0.0, 0.0, 1.0],
    },
];
const PENTAGON_INDICES: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4];

// Rectange
const RECTANGLE_VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.5, 0.25, 0.0],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [-0.5, -0.25, 0.0],
        color: [0.5, 0.5, 0.0],
    },
    Vertex {
        position: [0.5, -0.25, 0.0],
        color: [0.0, 0.5, 0.5],
    },
    Vertex {
        position: [0.5, 0.25, 0.0],
        color: [0.0, 0.0, 1.0],
    },
];
const RECTANGLE_INDICES: &[u16] = &[0, 1, 3, 1, 2, 3];

// Trapezoid
const TRAPEZOID_VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.25, 0.5, 0.0],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [-0.5, -0.5, 0.0],
        color: [0.5, 0.5, 0.0],
    },
    Vertex {
        position: [0.5, -0.5, 0.0],
        color: [0.0, 0.5, 0.5],
    },
    Vertex {
        position: [0.25, 0.5, 0.0],
        color: [0.0, 0.0, 1.0],
    },
];
const TRAPEZOID_INDICES: &[u16] = &[0, 1, 2, 0, 2, 3];

// Parallelogram
const PARALLELOGRAM_VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.25, 0.5, 0.0],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [-0.5, -0.5, 0.0],
        color: [0.5, 0.5, 0.0],
    },
    Vertex {
        position: [0.25, -0.5, 0.0],
        color: [0.0, 0.5, 0.5],
    },
    Vertex {
        position: [0.5, 0.5, 0.0],
        color: [0.0, 0.0, 1.0],
    },
];
const PARALLELOGRAM_INDICES: &[u16] = &[0, 1, 2, 0, 2, 3];
