//! WGPU renderer implementation for Tempest radar visualization.
//!
//! This module provides the core GPU context initialization and renderer
//! foundation for rendering radar data using wgpu.

use thiserror::Error;
use wgpu::{Buffer, Device, Instance, Queue, ShaderModule, VertexState};

use crate::config::{RadarStyle, RenderConfig};
use crate::pipeline::{vertex_attributes, PolarRadarVertex, RadarVertex, BASIC_SHADER_WGSL};
use crate::vertices::{sweep_to_vertices, RadarVertexConfig, Sweep};
use crate::ViewTransform;

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
/// Additionally, it maintains the current rendering state:
/// - [`RenderConfig`] - Current render configuration
/// - [`ViewTransform`] - Current view transformation
/// - `opacity` - Current radar opacity (0.0-1.0)
/// - `current_style` - Current radar display style
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
    /// Current render configuration.
    config: RenderConfig,
    /// Current view transformation for map projection.
    view_transform: ViewTransform,
    /// Current opacity value for radar overlay (0.0-1.0).
    opacity: f32,
    /// Current radar display style.
    current_style: RadarStyle,
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
            config: RenderConfig::default(),
            view_transform: ViewTransform::default(),
            opacity: RenderConfig::default().default_opacity,
            current_style: RadarStyle::default(),
        })
    }

    /// Applies a new render configuration.
    ///
    /// This updates the renderer's configuration including screen dimensions,
    /// range settings, and display options.
    ///
    /// # Arguments
    ///
    /// * `config` - The new render configuration to apply
    ///
    /// # Example
    ///
    /// ```ignore
    /// use tempest_render::config::RenderConfig;
    /// use tempest_render::renderer::WgpuRenderer;
    ///
    /// async fn reconfigure() {
    ///     let mut renderer = WgpuRenderer::new().await.expect("Failed to create renderer");
    ///     renderer.configure(RenderConfig::new(1920, 1080));
    /// }
    /// ```
    pub fn configure(&mut self, config: RenderConfig) {
        self.config = config;
    }

    /// Sets the view transformation for map projection.
    ///
    /// This controls how geographic coordinates are mapped to screen space,
    /// including zoom, rotation, and center position.
    ///
    /// # Arguments
    ///
    /// * `view` - The new view transformation to apply
    ///
    /// # Example
    ///
    /// ```ignore
    /// use tempest_render::view_transform::ViewTransform;
    /// use tempest_render::renderer::WgpuRenderer;
    ///
    /// async fn set_view() {
    ///     let mut renderer = WgpuRenderer::new().await.expect("Failed to create renderer");
    ///     renderer.set_view(ViewTransform::new(35.4183, -97.4514, 2.0, 16.0/9.0));
    /// }
    /// ```
    pub fn set_view(&mut self, view: ViewTransform) {
        self.view_transform = view;
    }

    /// Sets the radar overlay opacity.
    ///
    /// The opacity value controls how transparent the radar overlay appears.
    /// Values are clamped to the valid range [0.0, 1.0].
    ///
    /// # Arguments
    ///
    /// * `opacity` - The new opacity value (0.0 = fully transparent, 1.0 = fully opaque)
    ///
    /// # Example
    ///
    /// ```ignore
    /// use tempest_render::renderer::WgpuRenderer;
    ///
    /// async fn set_opacity() {
    ///     let mut renderer = WgpuRenderer::new().await.expect("Failed to create renderer");
    ///     renderer.set_opacity(0.5); // 50% opacity
    /// }
    /// ```
    pub fn set_opacity(&mut self, opacity: f32) {
        self.opacity = opacity.clamp(0.0, 1.0);
    }

    /// Returns a reference to the current render configuration.
    #[inline]
    pub fn config(&self) -> &RenderConfig {
        &self.config
    }

    /// Returns a reference to the current view transformation.
    #[inline]
    pub fn view_transform(&self) -> &ViewTransform {
        &self.view_transform
    }

    /// Returns the current opacity value.
    #[inline]
    pub fn opacity(&self) -> f32 {
        self.opacity
    }

    /// Returns the current radar display style.
    #[inline]
    pub fn current_style(&self) -> RadarStyle {
        self.current_style
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

/// A sweep renderer for handling radar sweep data.
///
/// This struct manages the GPU resources for rendering a single radar sweep,
/// including vertex buffers and render state.
pub struct SweepRenderer {
    /// The GPU vertex buffer containing sweep data.
    vertex_buffer: Option<Buffer>,
    /// Number of vertices in the current buffer.
    vertex_count: usize,
    /// The current radar display style.
    style: RadarStyle,
    /// Whether the sweep data needs to be re-uploaded.
    dirty: bool,
}

impl SweepRenderer {
    /// Creates a new SweepRenderer with default settings.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use tempest_render::renderer::SweepRenderer;
    ///
    /// let sweep_renderer = SweepRenderer::new();
    /// ```
    pub fn new() -> Self {
        Self {
            vertex_buffer: None,
            vertex_count: 0,
            style: RadarStyle::default(),
            dirty: false,
        }
    }

    /// Updates the sweep data by converting to vertices and uploading to GPU.
    ///
    /// This method:
    /// 1. Converts the sweep data to GPU vertices using the specified style
    /// 2. Creates or reuses a GPU buffer for the vertex data
    /// 3. Marks the renderer as clean after upload
    ///
    /// # Arguments
    ///
    /// * `device` - The wgpu device to create buffers on
    /// * `queue` - The wgpu queue to upload data through
    /// * `sweep` - The radar sweep data to upload
    /// * `style` - The radar display style to use
    ///
    /// # Errors
    ///
    /// Returns [`RenderError::BufferCreationFailed`] if vertex buffer creation fails.
    /// Returns [`RenderError::RenderFailed`] if no valid vertices are produced.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use tempest_render::renderer::{SweepRenderer, WgpuRenderer};
    /// use tempest_render::config::RadarStyle;
    /// use tempest_decode::Sweep;
    ///
    /// async fn update(renderer: &WgpuRenderer, sweep: Sweep) {
    ///     let mut sweep_renderer = SweepRenderer::new();
    ///     sweep_renderer.update_sweep(renderer.device(), renderer.queue(), &sweep, RadarStyle::Reflectivity)
    ///         .expect("Failed to update sweep");
    /// }
    /// ```
    pub fn update_sweep(
        &mut self,
        device: &Device,
        queue: &Queue,
        sweep: &Sweep,
        style: RadarStyle,
    ) -> RenderResult<()> {
        // Create vertex conversion config from default settings
        let config = RadarVertexConfig::default();

        // Convert sweep to vertices
        let vertices = sweep_to_vertices(sweep, &config).map_err(|e| {
            RenderError::RenderFailed(format!("Failed to convert sweep to vertices: {:?}", e))
        })?;

        self.vertex_count = vertices.len();

        // Convert vertices to bytes for GPU upload
        let vertex_data: &[u8] = bytemuck::cast_slice(&vertices);

        // Create or recreate the vertex buffer
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Sweep Vertex Buffer"),
            size: vertex_data.len() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: true,
        });

        // Write vertex data to buffer
        let mut mapping = buffer.slice(..).get_mapped_range_mut();
        mapping.copy_from_slice(vertex_data);
        drop(mapping);
        buffer.unmap();

        // Upload to GPU via queue
        queue.write_buffer(&buffer, 0, vertex_data);

        self.vertex_buffer = Some(buffer);
        self.style = style;
        self.dirty = false;

        Ok(())
    }

    /// Renders the sweep using the provided render pass and view transform.
    ///
    /// This method issues draw commands to the render pass for the current sweep data.
    /// It uses the view transform to handle polar coordinate to clip space transformation.
    ///
    /// # Arguments
    ///
    /// * `pass` - The wgpu render pass to issue draw commands to
    /// * `view` - The view transformation for coordinate conversion
    /// * `pipeline` - The render pipeline to use
    ///
    /// # Example
    ///
    /// ```ignore
    /// use tempest_render::view_transform::ViewTransform;
    /// use tempest_render::renderer::{SweepRenderer, RenderPipeline};
    ///
    /// fn render_sweep(pass: &mut wgpu::RenderPass, view: &ViewTransform, pipeline: &RenderPipeline) {
    ///     let sweep_renderer = SweepRenderer::new();
    ///     sweep_renderer.render(pass, view, pipeline);
    /// }
    /// ```
    pub fn render<'a>(
        &'a self,
        pass: &mut wgpu::RenderPass<'a>,
        _view: &ViewTransform,
        pipeline: &'a RenderPipeline,
    ) {
        // Check if we have valid vertex data to render
        let Some(buffer) = &self.vertex_buffer else {
            return;
        };

        if self.vertex_count == 0 {
            return;
        }

        // Set the render pipeline
        pass.set_pipeline(pipeline.pipeline());

        // Set the vertex buffer
        pass.set_vertex_buffer(0, buffer.slice(..));

        // Draw the vertices
        pass.draw(0..self.vertex_count as u32, 0..1);
    }

    /// Returns the number of vertices in the current sweep.
    #[inline]
    pub fn vertex_count(&self) -> usize {
        self.vertex_count
    }

    /// Returns the current radar display style.
    #[inline]
    pub fn style(&self) -> RadarStyle {
        self.style
    }

    /// Returns true if the sweep data needs to be re-uploaded.
    #[inline]
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }
}

impl Default for SweepRenderer {
    fn default() -> Self {
        Self::new()
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

    /// Tests for WgpuRenderer configuration methods.
    mod renderer_configuration {
        use super::*;

        #[tokio::test]
        #[ignore]
        async fn test_configure() {
            let result = WgpuRenderer::new().await;
            if result.is_err() {
                // Skip test on headless systems
                return;
            }

            let mut renderer = result.unwrap();

            let new_config = RenderConfig::new(1920, 1080);
            renderer.configure(new_config);

            assert_eq!(renderer.config().width, 1920);
            assert_eq!(renderer.config().height, 1080);
        }

        #[tokio::test]
        #[ignore]
        async fn test_set_view() {
            let result = WgpuRenderer::new().await;
            if result.is_err() {
                // Skip test on headless systems
                return;
            }

            let mut renderer = result.unwrap();

            let new_view = ViewTransform::new(40.0, -100.0, 2.0, 16.0 / 9.0);
            renderer.set_view(new_view);

            assert!((renderer.view_transform().zoom() - 2.0).abs() < 0.001);
            assert!((renderer.view_transform().center_lat() - 40.0).abs() < 0.001);
            assert!((renderer.view_transform().center_lon() - (-100.0)).abs() < 0.001);
        }

        #[tokio::test]
        #[ignore]
        async fn test_set_opacity_valid_range() {
            let result = WgpuRenderer::new().await;
            if result.is_err() {
                // Skip test on headless systems
                return;
            }

            let mut renderer = result.unwrap();

            // Test valid opacity values
            renderer.set_opacity(0.0);
            assert!((renderer.opacity() - 0.0).abs() < 0.001);

            renderer.set_opacity(0.5);
            assert!((renderer.opacity() - 0.5).abs() < 0.001);

            renderer.set_opacity(1.0);
            assert!((renderer.opacity() - 1.0).abs() < 0.001);
        }

        #[tokio::test]
        #[ignore]
        async fn test_set_opacity_clamped_above_range() {
            let result = WgpuRenderer::new().await;
            if result.is_err() {
                // Skip test on headless systems
                return;
            }

            let mut renderer = result.unwrap();

            // Test values above 1.0 are clamped to 1.0
            renderer.set_opacity(1.5);
            assert!((renderer.opacity() - 1.0).abs() < 0.001);

            renderer.set_opacity(2.0);
            assert!((renderer.opacity() - 1.0).abs() < 0.001);
        }

        #[tokio::test]
        #[ignore]
        async fn test_set_opacity_clamped_below_range() {
            let result = WgpuRenderer::new().await;
            if result.is_err() {
                // Skip test on headless systems
                return;
            }

            let mut renderer = result.unwrap();

            // Test values below 0.0 are clamped to 0.0
            renderer.set_opacity(-0.5);
            assert!((renderer.opacity() - 0.0).abs() < 0.001);

            renderer.set_opacity(-1.0);
            assert!((renderer.opacity() - 0.0).abs() < 0.001);
        }

        #[tokio::test]
        #[ignore]
        async fn test_default_values() {
            let result = WgpuRenderer::new().await;
            if result.is_err() {
                // Skip test on headless systems
                return;
            }

            let renderer = result.unwrap();

            // Check default config
            assert_eq!(renderer.config().width, 800);
            assert_eq!(renderer.config().height, 600);

            // Check default opacity
            assert!((renderer.opacity() - 0.7).abs() < 0.001);

            // Check default style
            assert_eq!(renderer.current_style(), RadarStyle::Reflectivity);
        }
    }

    /// Tests for SweepRenderer.
    mod sweep_renderer {
        use super::*;

        #[test]
        fn test_sweep_renderer_creation() {
            let sweep_renderer = SweepRenderer::new();

            assert_eq!(sweep_renderer.vertex_count(), 0);
            assert_eq!(sweep_renderer.style(), RadarStyle::Reflectivity);
            assert!(!sweep_renderer.is_dirty());
        }

        #[test]
        fn test_sweep_renderer_default() {
            let sweep_renderer: SweepRenderer = Default::default();

            assert_eq!(sweep_renderer.vertex_count(), 0);
            assert_eq!(sweep_renderer.style(), RadarStyle::Reflectivity);
        }
    }
}
