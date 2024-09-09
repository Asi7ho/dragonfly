pub mod vertex;

pub use vertex::Figure;
pub use vertex::Vertex;

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
#[macro_export]
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
#[macro_export]
macro_rules! circle_indices {
    ($num_segments:expr) => {{
        const NUM_SEGMENTS: usize = $num_segments;

        let indices: Vec<u16> = (1..(NUM_SEGMENTS + 1) as u16)
            .flat_map(|i| [0, i, i + 1])
            .collect();

        indices
    }};
}

/// Defines the vertices and indices for a triangle.
///
/// The triangle is defined by three vertices, each with a position and a color.
/// The vertices are arranged in a counter-clockwise direction.
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

/// Defines the vertices and indices for a pentagon.
///
/// The pentagon is defined by five vertices, each with a position and a color.
/// The vertices are arranged in a counter-clockwise direction.
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

/// Defines the vertices and indices for a rectangle.
///
/// The rectangle is defined by four vertices, each with a position and a color.
/// The vertices are arranged in a counter-clockwise direction.
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

/// Defines the vertices and indices for a trapezoid.
///
/// The trapezoid is defined by four vertices, each with a position and a color.
/// The vertices are arranged in a counter-clockwise direction.
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

/// Defines the vertices and indices for a parallelogram.
///
/// The parallelogram is defined by four vertices, each with a position and a
/// color.
/// The vertices are arranged in a counter-clockwise direction.
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
