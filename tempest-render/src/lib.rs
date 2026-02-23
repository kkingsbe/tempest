//! Tempest Render - GPU radar rendering using wgpu
//!
//! This crate provides GPU-accelerated rendering for weather radar data
//! using the wgpu graphics API.

pub mod config;
pub mod pipeline;
pub mod renderer;
pub mod vertices;
pub mod view_transform;

// Re-export the main renderer types for convenient access
pub use renderer::{RenderError, RenderPipeline, RenderResult, SweepRenderer, WgpuRenderer};

// Re-export pipeline types
pub use pipeline::{
    PolarRadarVertex, RadarValueType, POLAR_RADAR_VERTEX_STRIDE, RADAR_VERTEX_STRIDE,
};

// Re-export vertex conversion utilities
pub use vertices::{
    projected_sweep_to_vertices, RadarVertexConfig, VertexConversionError, VertexResult,
};

// Re-export view transform types
pub use view_transform::{ViewTransform, ViewUniforms};

// Re-export config types
pub use config::{RadarStyle, RenderConfig};
