pub mod vertex;

pub use vertex::Vertex;

/// Represents a geometric figure that can be rendered.
///
/// The `Figure` enum defines various geometric shapes that can be used for
/// rendering.
#[derive(Debug, Default)]
pub enum Figure {
    #[default]
    Triangle,
    Pentagon,
    Rectangle,
    Trapezoid,
    Parallelogram,
    Circle(u32),
}

/// A trait representing a mesh, which is a collection of vertices and indices.
///
/// Implementors of this trait can provide their own methods for retrieving the vertices and indices.
pub trait Mesh {
    /// Returns a vector of vertices that make up the mesh.
    fn get_vertices(&self) -> Vec<Vertex>;

    /// Returns a vector of indices that define the order of vertices to be used for rendering.
    fn get_indices(&self) -> Vec<u16>;
}

/// Implementation of the `Mesh` trait for the `Figure` enum.
///
/// This implementation allows the `Figure` enum to be used as a mesh, providing
/// methods for retrieving the vertices and indices that make up the geometric
/// figure.
impl Mesh for Figure {
    fn get_vertices(&self) -> Vec<Vertex> {
        match self {
            Figure::Triangle => vec![
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
            ],
            Figure::Pentagon => vec![
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
            ],
            Figure::Rectangle => vec![
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
            ],
            Figure::Trapezoid => vec![
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
            ],
            Figure::Parallelogram => vec![
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
            ],
            Figure::Circle(num_segments) => {
                const TWO_PI: f32 = 2.0 * std::f32::consts::PI;

                let vertices: Vec<Vertex> = std::iter::once(Vertex {
                    position: [0.0, 0.0, 0.0],
                    color: [0.5, 0.5, 0.5],
                })
                .chain((0..(num_segments + 1)).map(|i| {
                    let angle = i as f32 * TWO_PI / *num_segments as f32;
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
            }
        }
    }

    fn get_indices(&self) -> Vec<u16> {
        match self {
            Figure::Triangle => vec![0, 1, 2],
            Figure::Pentagon => vec![0, 1, 4, 1, 2, 4, 2, 3, 4],
            Figure::Rectangle | Figure::Trapezoid | Figure::Parallelogram => vec![0, 1, 3, 1, 2, 3],
            Figure::Circle(num_segments) => {
                let indices: Vec<u16> = (1..(num_segments + 1) as u16)
                    .flat_map(|i| [0, i, i + 1])
                    .collect();

                indices
            }
        }
    }
}

impl Figure {
    /// Returns the figure at the given index.
    ///
    /// If the index is not in the range 0..4, the default figure (Triangle) is
    /// returned.
    pub fn get_figure(i: u8) -> Self {
        match i {
            0 => Figure::Triangle,
            1 => Figure::Pentagon,
            2 => Figure::Rectangle,
            3 => Figure::Trapezoid,
            4 => Figure::Parallelogram,
            5 => Figure::Circle(64),
            _ => Figure::Triangle,
        }
    }
}
