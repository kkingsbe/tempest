//! Tempest Render - GPU radar rendering using wgpu
//!
//! This crate provides GPU-accelerated rendering for weather radar data
//! using the wgpu graphics API.

pub mod pipeline;
pub mod renderer;
pub mod vertices;

// Re-export the main renderer types for convenient access
pub use renderer::{RenderError, RenderPipeline, RenderResult, WgpuRenderer};

// Re-export pipeline types
pub use pipeline::{
    PolarRadarVertex, RadarValueType, POLAR_RADAR_VERTEX_STRIDE, RADAR_VERTEX_STRIDE,
};

// Re-export vertex conversion utilities
pub use vertices::{RadarVertexConfig, VertexConversionError, VertexResult};
