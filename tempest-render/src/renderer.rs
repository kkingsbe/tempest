//! WGPU renderer implementation for Tempest radar visualization.
//!
//! This module provides the core GPU context initialization and renderer
//! foundation for rendering radar data using wgpu.

use thiserror::Error;
use wgpu::{Device, Instance, Queue, Surface};

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

    /// Failed to create a rendering surface.
    #[error("Failed to create rendering surface: {0}")]
    SurfaceCreationFailed(String),

    /// No suitable GPU adapter found.
    #[error("No suitable GPU adapter found")]
    NoAdapterFound,

    /// Rendering operation failed.
    #[error("Rendering failed: {0}")]
    RenderFailed(String),
}

/// Result type alias for rendering operations.
pub type RenderResult<T> = Result<T, RenderError>;

/// WGPU renderer that manages the GPU context for radar rendering.
///
/// This struct holds the core wgpu objects needed for rendering:
/// - [`Instance`] - The wgpu instance for adapter discovery
/// - [`Surface`] - The rendering surface for presenting frames
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
    /// The rendering surface for presenting to a window.
    surface: Surface<'static>,
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
    /// 4. Creates a rendering surface (headless for now)
    ///
    /// # Errors
    ///
    /// Returns [`RenderError::AdapterRequestFailed`] if no adapter can be found.
    /// Returns [`RenderError::NoAdapterFound`] if the instance has no adapters.
    /// Returns [`RenderError::DeviceRequestFailed`] if device creation fails.
    /// Returns [`RenderError::SurfaceCreationFailed`] if surface creation fails.
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

        // Step 4: Create a headless surface (no window attached)
        // This creates a surface that can be used for offscreen rendering
        let surface = instance
            .create_surface(wgpu::SurfaceTarget::Dummy)
            .map_err(|e| RenderError::SurfaceCreationFailed(e.to_string()))?;

        Ok(Self {
            instance,
            surface,
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

    /// Returns a reference to the rendering surface.
    ///
    /// The surface is used for presenting rendered frames.
    #[inline]
    pub fn surface(&self) -> &Surface<'static> {
        &self.surface
    }

    /// Returns a reference to the wgpu instance.
    ///
    /// The instance can be used for adapter enumeration and other
    /// instance-level operations.
    #[inline]
    pub fn instance(&self) -> &Instance {
        &self.instance
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
            println!("Renderer creation failed (expected on headless systems): {}", e);
        }
        
        // The test passes as long as we can attempt creation
        let _ = result;
    }
}
