use std::sync::Arc;

use dragonfly::vertex::{self, Mesh, Vertex};
use wgpu::util::DeviceExt;
use winit::window::Window;

/// Graphics context for rendering.
///
/// This type holds all the necessary data to render a `Figure` on a window
/// using the `wgpu` library.
#[derive(Debug)]
pub struct Context {
    /// The surface to render on.
    pub surface: wgpu::Surface<'static>,
    /// The device to use for rendering.
    pub device: wgpu::Device,
    /// The queue to use for rendering.
    pub queue: wgpu::Queue,
    /// The surface configuration.
    pub config: wgpu::SurfaceConfiguration,
    /// The size of the window.
    pub size: winit::dpi::PhysicalSize<u32>,
    /// The render pipeline.
    pub render_pipeline: wgpu::RenderPipeline,

    /// The index of the current figure.
    pub fig_idx: u8,

    /// The vertex buffer.
    pub vertex_buffer: wgpu::Buffer,
    /// The number of vertices in the vertex buffer.
    pub num_vertices: u32,

    /// The index buffer.
    pub index_buffer: wgpu::Buffer,
    /// The number of indices in the index buffer.
    pub num_indices: u32,
}

impl Context {
    /// Creates a new graphics context for rendering on the given window.
    ///
    /// The context consists of a `wgpu` instance, surface, device, queue, and
    /// surface configuration. Additionally, it creates a shader module, render
    /// pipeline layout, and render pipeline.
    ///
    /// The context is configured for the initial window size and the first
    /// figure.
    pub async fn new(window: &Arc<Window>) -> Self {
        let size = window.inner_size();

        // Create a new instance that take the default backend for the device.
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        // Create a new surface for rendering.
        let surface = instance
            .create_surface(window.clone())
            .expect("Failed to create surface");

        // Request a graphics adapter from the wgpu instance.
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("Failed to create adapter");

        // Request a logical device and command queue from the adapter with
        // no extra features and default limits.
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    label: None,
                    memory_hints: wgpu::MemoryHints::default(),
                },
                None, // Trace path
            )
            .await
            .unwrap();

        // Extract the supported/prefered format for the surface.
        let capabilities = surface.get_capabilities(&adapter);
        let surface_format = capabilities
            .formats
            .iter()
            .copied()
            .find(wgpu::TextureFormat::is_srgb)
            .or_else(|| capabilities.formats.first().copied())
            .expect("Failed to get preferred format");

        // Configures the surface with the correct format for rendering.
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::default(),
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
            desired_maximum_frame_latency: 1,
        };

        // Create a shader module from a shader written in WGSL.
        let shader = device.create_shader_module(wgpu::include_wgsl!("../shaders/shader.wgsl"));

        // Create the render pipeline layout.
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        // Create the render pipeline.
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&render_pipeline_layout),
            // Read vertex shader
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            // Read fragment shader
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            // Set the topology
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        // Set the initial figure
        let fig_idx = 0;
        let figure = vertex::Figure::get_figure(fig_idx);
        let vertices = figure.get_vertices();
        let indices = figure.get_indices();

        // Create the vertex and index buffers
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        Self {
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,

            fig_idx,

            vertex_buffer,
            num_vertices: vertices.len() as u32,

            index_buffer,
            num_indices: indices.len() as u32,
        }
    }

    /// Resizes the graphics context for the given window size.
    ///
    /// The `device` and `surface` fields are updated for the new window size.
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        // Update config and surface for new window size.
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    /// Renders the current figure on the window.
    ///
    /// This method acquires the current frame from the window, clears the
    /// render target, sets up the vertex and index buffers, renders the
    /// figure, and presents the frame.
    ///
    /// # Errors
    ///
    /// Returns an error if the current frame could not be acquired from the
    /// window.
    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        // Get current frame.
        let frame = self
            .surface
            .get_current_texture()
            .expect("Failed to get texture");

        // Get current texture view.
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // Create a command encoder to transfer operations.
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        // Clear render
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            // Render the figure
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }

        // Submit the operations
        self.queue.submit(std::iter::once(encoder.finish()));
        frame.present();

        Ok(())
    }
}
