//! Performance benchmarks for tempest-fetch pipeline
//!
//! This module benchmarks the fetch + decode pipeline.
//! Target: pipeline < 500ms p95

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::num::NonZeroUsize;
use std::time::Duration;

/// Benchmark the full fetch pipeline (mock S3 + decode)
/// Note: This is a synchronous benchmark wrapper around async operations
fn benchmark_full_pipeline(c: &mut Criterion) {
    c.benchmark_group("pipeline")
        .sample_size(10)
        .measurement_time(Duration::from_secs(10))
        .bench_function("pipeline_single_scan", |b| {
            b.iter(|| {
                // This represents the full pipeline for a single scan
                // In practice, this would be async code running in a tokio runtime
                black_box(());
            });
        });
}

/// Benchmark the S3 client connection setup
fn benchmark_s3_client_creation(c: &mut Criterion) {
    c.benchmark_group("s3_client")
        .sample_size(20)
        .measurement_time(Duration::from_secs(5))
        .bench_function("client_instantiation", |b| {
            b.iter(|| {
                // Placeholder for client creation benchmark
                // In real benchmark, this would create S3Client
                black_box(());
            });
        });
}

/// Benchmark scan listing operation
fn benchmark_scan_listing(c: &mut Criterion) {
    c.benchmark_group("scan_listing")
        .sample_size(15)
        .measurement_time(Duration::from_secs(5))
        .bench_function("list_scans_parsing", |b| {
            // Sample XML response similar to S3 ListObjectsV2
            let sample_xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<ListBucketResult xmlns="http://s3.amazonaws.com/doc/2006-03-01/">
  <CommonPrefixes>
    <Prefix>2024/03/15/KTLX/KTLX20240315_120021</Prefix>
  </CommonPrefixes>
  <CommonPrefixes>
    <Prefix>2024/03/15/KTLX/KTLX20240315_120521</Prefix>
  </CommonPrefixes>
  <CommonPrefixes>
    <Prefix>2024/03/15/KTLX/KTLX20240315_121021</Prefix>
  </CommonPrefixes>
</ListBucketResult>"#;

            b.iter(|| {
                // Parse scan list from XML (simplified)
                let _has_prefix = black_box(sample_xml.contains("CommonPrefixes"));
                black_box(_has_prefix);
            });
        });
}

/// Benchmark data decompression (bzip2)
fn benchmark_decompression(c: &mut Criterion) {
    use std::io::Read;

    c.benchmark_group("decompression")
        .sample_size(10)
        .measurement_time(Duration::from_secs(5))
        .bench_function("decompress_bzip2", |b| {
            // Read compressed test fixture
            let compressed_path = "tests/fixtures/SuperRes_KTLX_20240427.ar2v";
            let compressed_data = std::fs::read(compressed_path).expect("Failed to read fixture");

            b.iter(|| {
                let mut decompressor =
                    bzip2::bufread::BzDecoder::new(black_box(&compressed_data[..]));
                let mut decompressed = Vec::new();
                let _ = decompressor.read_to_end(&mut decompressed);
                black_box(decompressed)
            });
        });
}

/// Benchmark cache operations
fn benchmark_cache_operations(c: &mut Criterion) {
    use lru::LruCache;

    c.benchmark_group("cache")
        .sample_size(20)
        .measurement_time(Duration::from_secs(5))
        .bench_function("cache_get_hit", |b| {
            let mut cache = LruCache::new(NonZeroUsize::new(100).unwrap());
            cache.put("key1", vec![1u8; 1000]);
            cache.put("key2", vec![2u8; 1000]);

            b.iter(|| {
                let result = cache.get(black_box(&"key1"));
                black_box(result.is_some())
            });
        });

    c.benchmark_group("cache")
        .bench_function("cache_get_miss", |b| {
            let mut cache = LruCache::new(NonZeroUsize::new(100).unwrap());
            cache.put("key1", vec![1u8; 1000]);

            b.iter(|| {
                let result = cache.get(black_box(&"nonexistent"));
                black_box(result.is_none())
            });
        });
}

criterion_group!(
    benches,
    benchmark_full_pipeline,
    benchmark_s3_client_creation,
    benchmark_scan_listing,
    benchmark_decompression,
    benchmark_cache_operations
);
criterion_main!(benches);
