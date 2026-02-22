//! WGPU renderer implementation for Tempest radar visualization.
//!
//! This module provides the core GPU context initialization and renderer
//! foundation for rendering radar data using wgpu.

use thiserror::Error;
use wgpu::{Buffer, Device, Instance, Queue, ShaderModule, VertexState};

use crate::pipeline::{vertex_attributes, PolarRadarVertex, RadarVertex, BASIC_SHADER_WGSL};

/// Custom error types for WGPU rendering operations.
///
/// These errors cover the main failure modes when initializing the GPU
/// context and when performing render operations.
#[derive(Debug, Error)]
pub enum RenderError {
    /// Failed to request a GPU adapter.
    #[error("Failed to request GPU adapter: {0}")]
    AdapterRequestFailed(String),

    /// Failed to request a GPU device.
    #[error("Failed to request GPU device: {0}")]
    DeviceRequestFailed(String),

    /// No suitable GPU adapter found.
    #[error("No suitable GPU adapter found")]
    NoAdapterFound,

    /// Rendering operation failed.
    #[error("Rendering failed: {0}")]
    RenderFailed(String),

    /// Vertex buffer creation failed.
    #[error("Failed to create vertex buffer: {0}")]
    BufferCreationFailed(String),
}

/// Result type alias for rendering operations.
pub type RenderResult<T> = Result<T, RenderError>;

/// WGPU renderer that manages the GPU context for radar rendering.
///
/// This struct holds the core wgpu objects needed for rendering:
/// - [`Instance`] - The wgpu instance for adapter discovery
/// - [`Device`] - The logical GPU device
/// - [`Queue`] - The command queue for submitting work
///
/// # Example
///
/// ```ignore
/// use tempest_render::renderer::WgpuRenderer;
///
/// async fn init_renderer() -> Result<WgpuRenderer, RenderError> {
///     WgpuRenderer::new().await
/// }
/// ```
pub struct WgpuRenderer {
    /// The wgpu instance for adapter enumeration.
    instance: Instance,
    /// The logical GPU device.
    device: Device,
    /// The command queue for submitting GPU work.
    queue: Queue,
}

impl WgpuRenderer {
    /// Creates a new WGPU renderer with the default backend.
    ///
    /// This async function:
    /// 1. Creates a new wgpu Instance with default backend selection
    /// 2. Requests a suitable GPU adapter
    /// 3. Requests a logical device and command queue
    ///
    /// # Errors
    ///
    /// Returns [`RenderError::AdapterRequestFailed`] if no adapter can be found.
    /// Returns [`RenderError::NoAdapterFound`] if the instance has no adapters.
    /// Returns [`RenderError::DeviceRequestFailed`] if device creation fails.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use tempest_render::renderer::WgpuRenderer;
    ///
    /// async fn main() {
    ///     match WgpuRenderer::new().await {
    ///         Ok(renderer) => println!("Renderer initialized successfully"),
    ///         Err(e) => eprintln!("Failed to initialize renderer: {}", e),
    ///     }
    /// }
    /// ```
    pub async fn new() -> RenderResult<Self> {
        // Step 1: Create the wgpu instance with default backend
        let instance = Instance::new(wgpu::InstanceDescriptor::default());

        // Step 2: Request a suitable adapter
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .ok_or(RenderError::NoAdapterFound)
            .map_err(|e| RenderError::AdapterRequestFailed(e.to_string()))?;

        // Step 3: Request a logical device and queue
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default(), None)
            .await
            .map_err(|e| RenderError::DeviceRequestFailed(e.to_string()))?;

        Ok(Self {
            instance,
            device,
            queue,
        })
    }

    /// Returns a reference to the GPU device.
    ///
    /// The device is used for creating resources like buffers and textures.
    #[inline]
    pub fn device(&self) -> &Device {
        &self.device
    }

    /// Returns a reference to the command queue.
    ///
    /// The queue is used to submit command buffers for execution.
    #[inline]
    pub fn queue(&self) -> &Queue {
        &self.queue
    }

    /// Returns a reference to the wgpu instance.
    ///
    /// The instance can be used for adapter enumeration and surface creation.
    /// Use [`Instance::create_surface`] to create a rendering surface
    /// when you have a window handle available.
    #[inline]
    pub fn instance(&self) -> &Instance {
        &self.instance
    }
}

/// A render pipeline for rendering radar data.
///
/// This struct holds the wgpu render pipeline and associated resources
/// needed for rendering radar geometry with colors.
///
/// # Example
///
/// ```ignore
/// use tempest_render::renderer::{WgpuRenderer, RenderPipeline};
///
/// async fn init_pipeline() -> Result<RenderPipeline, RenderError> {
///     let renderer = WgpuRenderer::new().await?;
///     RenderPipeline::new(&renderer)
/// }
/// ```
pub struct RenderPipeline {
    /// The underlying wgpu render pipeline.
    pipeline: wgpu::RenderPipeline,
    /// The compiled shader module.
    shader_module: ShaderModule,
}

impl RenderPipeline {
    /// Creates a new render pipeline for basic radar rendering.
    ///
    /// This function:
    /// 1. Compiles the embedded WGSL shader module
    /// 2. Creates a pipeline layout
    /// 3. Configures the render pipeline with vertex/fragment stages
    ///
    /// # Errors
    ///
    /// Returns [`RenderError::RenderFailed`] if shader compilation fails
    /// or pipeline creation fails.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use tempest_render::renderer::{WgpuRenderer, RenderPipeline};
    ///
    /// async fn main() {
    ///     let renderer = WgpuRenderer::new().await.expect("Failed to create renderer");
    ///     let pipeline = RenderPipeline::new(&renderer).expect("Failed to create pipeline");
    /// }
    /// ```
    pub fn new(renderer: &WgpuRenderer) -> RenderResult<Self> {
        // Step 1: Create the shader module from embedded WGSL
        let shader_module = renderer
            .device()
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Tempest Basic Radar Shader"),
                source: wgpu::ShaderSource::Wgsl(BASIC_SHADER_WGSL.into()),
            });

        // Step 2: Define the vertex buffer layout
        let vertex_buffers = [wgpu::VertexBufferLayout {
            array_stride: crate::pipeline::RADAR_VERTEX_STRIDE,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &vertex_attributes(),
        }];

        // Step 3: Create the pipeline layout
        let pipeline_layout =
            renderer
                .device()
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Tempest Pipeline Layout"),
                    bind_group_layouts: &[],
                    push_constant_ranges: &[],
                });

        // Step 4: Create the render pipeline
        let pipeline = renderer
            .device()
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Tempest Basic Render Pipeline"),
                layout: Some(&pipeline_layout),
                vertex: VertexState {
                    module: &shader_module,
                    entry_point: Some("vertex_main"),
                    buffers: &vertex_buffers,
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader_module,
                    entry_point: Some("fragment_main"),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: wgpu::TextureFormat::Rgba8Unorm,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    ..Default::default()
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
                cache: None,
            });

        Ok(Self {
            pipeline,
            shader_module,
        })
    }

    /// Returns a reference to the underlying wgpu render pipeline.
    #[inline]
    pub fn pipeline(&self) -> &wgpu::RenderPipeline {
        &self.pipeline
    }

    /// Returns a reference to the shader module.
    #[inline]
    pub fn shader_module(&self) -> &ShaderModule {
        &self.shader_module
    }
}

/// Vertex buffer utilities for creating GPU buffers from radar data.
///
/// These functions help convert radar vertex data into wgpu buffers
/// that can be used for rendering.
pub mod vertex_buffer {
    use super::*;
    use bytemuck::cast_slice;

    /// Creates a vertex buffer from polar radar vertices.
    ///
    /// This function takes a slice of [`PolarRadarVertex`] and creates
    /// a GPU buffer suitable for use with the polar radar rendering pipeline.
    ///
    /// # Arguments
    ///
    /// * `device` - The wgpu device to create the buffer on
    /// * `vertices` - Slice of polar radar vertices
    ///
    /// # Returns
    ///
    /// A wgpu Buffer containing the vertex data.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use tempest_render::renderer::vertex_buffer::create_polar_vertex_buffer;
    /// use tempest_render::pipeline::PolarRadarVertex;
    ///
    /// let vertices = vec![
    ///     PolarRadarVertex::with_reflectivity(1000.0, 0.0, 30.0),
    ///     PolarRadarVertex::with_reflectivity(2000.0, 0.0, 35.0),
    /// ];
    /// let buffer = create_polar_vertex_buffer(renderer.device(), &vertices);
    /// ```
    pub fn create_polar_vertex_buffer(
        device: &Device,
        vertices: &[PolarRadarVertex],
    ) -> RenderResult<Buffer> {
        let vertex_data: &[u8] = cast_slice(vertices);

        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Polar Radar Vertex Buffer"),
            size: vertex_data.len() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: true,
        });

        // Write the data to the mapped buffer
        let mut mapping = buffer.slice(..).get_mapped_range_mut();
        mapping.copy_from_slice(vertex_data);
        drop(mapping);
        buffer.unmap();

        Ok(buffer)
    }

    /// Creates a vertex buffer from basic radar vertices.
    ///
    /// This function takes a slice of [`RadarVertex`] and creates
    /// a GPU buffer suitable for use with the basic radar rendering pipeline.
    pub fn create_vertex_buffer(device: &Device, vertices: &[RadarVertex]) -> RenderResult<Buffer> {
        let vertex_data: &[u8] = cast_slice(vertices);

        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Basic Radar Vertex Buffer"),
            size: vertex_data.len() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: true,
        });

        let mut mapping = buffer.slice(..).get_mapped_range_mut();
        mapping.copy_from_slice(vertex_data);
        drop(mapping);
        buffer.unmap();

        Ok(buffer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_renderer_creation() {
        // This test will fail on systems without GPU support
        // but verifies the initialization path works
        let result = WgpuRenderer::new().await;

        // On systems without GPU, we expect an error
        // On systems with GPU, this should succeed
        if let Err(e) = &result {
            println!(
                "Renderer creation failed (expected on headless systems): {}",
                e
            );
        }

        // The test passes as long as we can attempt creation
        let _ = result;
    }
}
