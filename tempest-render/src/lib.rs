//! Tempest Render - GPU radar rendering using wgpu
//!
//! This crate provides GPU-accelerated rendering for weather radar data
//! using the wgpu graphics API.

pub mod renderer;

// Re-export the main renderer types for convenient access
pub use renderer::{RenderError, RenderResult, WgpuRenderer};
