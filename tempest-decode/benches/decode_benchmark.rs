//! Performance benchmarks for tempest-decode
//!
//! This module benchmarks the radar data decoding functionality.
//! Target: decode < 100ms per radial

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs;

/// Benchmark decoding a single radial from a real NEXRAD file
fn benchmark_decode_radial(c: &mut Criterion) {
    // Load test fixture data
    let fixture_path = "tests/fixtures/SuperRes_KTLX_20240427.ar2v";
    let data = fs::read(fixture_path).expect("Failed to read fixture file");
    
    c.benchmark_group("decode")
        .sample_size(10)
        .measurement_time(std::time::Duration::from_secs(5))
        .bench_function("decode_full_volume", |b| {
            b.iter(|| {
                // Decode the entire volume scan
                let result = tempest_decode::decode(black_box(&data));
                black_box(result)
            });
        });
}

/// Benchmark decoding a minimal radial dataset
fn benchmark_decode_minimal_radial(c: &mut Criterion) {
    // Create minimal radial data for faster benchmarking
    // This tests the core decoding path without file I/O
    
    // Build a minimal NEXRAD Message 31 radial
    let mut minimal_data = Vec::new();
    
    // Message size (4 bytes) - 24 bytes total
    minimal_data.extend_from_slice(&24u32.to_be_bytes());
    // Message type (2 bytes) - 31 for radial
    minimal_data.extend_from_slice(&31u16.to_be_bytes());
    // MJD date (2 bytes)
    minimal_data.extend_from_slice(&60000u16.to_be_bytes());
    // Time in ms (4 bytes)
    minimal_data.extend_from_slice(&43200000u32.to_be_bytes());
    // Station ID (4 bytes) - "KTLX"
    minimal_data.extend_from_slice(b"KTLX");
    // Volume scan number (2 bytes)
    minimal_data.extend_from_slice(&1u16.to_be_bytes());
    // VCP (2 bytes) - 212
    minimal_data.extend_from_slice(&212u16.to_be_bytes());
    // Elevation angle (4 bytes) - 0.5 degrees
    minimal_data.extend_from_slice(&0.5f32.to_be_bytes());
    // Number of gates (2 bytes) - 100 gates
    minimal_data.extend_from_slice(&100u16.to_be_bytes());
    // Gate range (2 bytes) - 250m
    minimal_data.extend_from_slice(&250u16.to_be_bytes());
    
    c.benchmark_group("decode_minimal")
        .sample_size(20)
        .measurement_time(std::time::Duration::from_secs(3))
        .bench_function("decode_single_radial_100gates", |b| {
            b.iter(|| {
                let result = tempest_decode::decode(black_box(&minimal_data));
                black_box(result)
            });
        });
}

/// Benchmark memory usage during decoding
fn benchmark_decode_memory(c: &mut Criterion) {
    let fixture_path = "tests/fixtures/SuperRes_KTLX_20240427.ar2v";
    let data = fs::read(fixture_path).expect("Failed to read fixture file");
    
    c.benchmark_group("memory")
        .bench_function("decode_with_allocation", |b| {
            b.iter(|| {
                let result = tempest_decode::decode(black_box(&data));
                // Force some allocation by accessing the result
                if let Ok(volume) = result {
                    black_box(volume.sweeps.len());
                }
            });
        });
}

criterion_group!(
    benches,
    benchmark_decode_radial,
    benchmark_decode_minimal_radial,
    benchmark_decode_memory
);
criterion_main!(benches);
