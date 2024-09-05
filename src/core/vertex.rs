use bytemuck;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
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

// Triangle
// pub const VERTICES: &[Vertex] = &[
//     Vertex {
//         position: [0.0, 0.5, 0.0],
//         color: [1.0, 0.0, 0.0],
//     },
//     Vertex {
//         position: [-0.5, -0.5, 0.0],
//         color: [0.0, 1.0, 0.0],
//     },
//     Vertex {
//         position: [0.5, -0.5, 0.0],
//         color: [0.0, 0.0, 1.0],
//     },
// ];

// Hexagon
// pub const VERTICES: &[Vertex] = &[
//     Vertex {
//         position: [-0.0868241, 0.49240386, 0.0],
//         color: [0.5, 0.0, 0.5],
//     },
//     Vertex {
//         position: [-0.49513406, 0.06958647, 0.0],
//         color: [0.5, 0.0, 0.5],
//     },
//     Vertex {
//         position: [-0.21918549, -0.44939706, 0.0],
//         color: [0.5, 0.0, 0.5],
//     },
//     Vertex {
//         position: [0.35966998, -0.3473291, 0.0],
//         color: [0.5, 0.0, 0.5],
//     },
//     Vertex {
//         position: [0.44147372, 0.2347359, 0.0],
//         color: [0.5, 0.0, 0.5],
//     },
// ];

// pub const INDICES: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4];

// Rectange
// pub const VERTICES: &[Vertex] = &[
//     Vertex {
//         position: [-0.5, 0.25, 0.0],
//         color: [0.5, 0.0, 0.5],
//     },
//     Vertex {
//         position: [-0.5, -0.25, 0.0],
//         color: [0.5, 0.0, 0.5],
//     },
//     Vertex {
//         position: [0.5, -0.25, 0.0],
//         color: [0.5, 0.0, 0.5],
//     },
//     Vertex {
//         position: [0.5, 0.25, 0.0],
//         color: [0.5, 0.0, 0.5],
//     },
// ];

// pub const INDICES: &[u16] = &[0, 1, 3, 1, 2, 3];

// Trapezoid
pub const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.25, 0.5, 0.0],
        color: [0.5, 0.0, 0.5],
    },
    Vertex {
        position: [-0.5, -0.5, 0.0],
        color: [0.5, 0.0, 0.5],
    },
    Vertex {
        position: [0.0, -0.5, 0.0],
        color: [0.5, 0.0, 0.5],
    },
    Vertex {
        position: [0.5, -0.5, 0.0],
        color: [0.5, 0.0, 0.5],
    },
    Vertex {
        position: [0.25, 0.5, 0.0],
        color: [0.5, 0.0, 0.5],
    },
];

pub const INDICES: &[u16] = &[0, 1, 2, 0, 2, 4, 2, 3, 4];
