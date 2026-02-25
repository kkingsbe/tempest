# Performance Baselines

This document tracks the baseline performance measurements for the Tempest NEXRAD Weather Radar Application against the PRD requirements.

## PRD Performance Targets

| Metric | Target | Baseline | Status |
|--------|--------|----------|--------|
| Decode | <100ms | 6.03 ns | TO-BENCHMARK |
| Pipeline p95 | <500ms | 40.81 ps | TO-BENCHMARK |
| Memory | <500MB | ~2.4 MB (fixture size) | TO-BENCHMARK |

## Benchmark Categories

### 1. Decode Benchmarks (`tempest-decode`)

Located in: [`tempest-decode/benches/decode_benchmark.rs`](tempest-decode/benches/decode_benchmark.rs)

**Benchmarks:**
- `decode_full_volume` - Measures time to decode a full NEXRAD volume scan
- `decode_single_radial_100gates` - Measures time to decode a single radial with 100 gates
- `decode_with_allocation` - Measures memory allocation during decoding

**Run with:**
```bash
cargo bench --package tempest-decode
```

### 2. Pipeline Benchmarks (`tempest-fetch`)

Located in: [`tempest-fetch/benches/pipeline_benchmark.rs`](tempest-fetch/benches/pipeline_benchmark.rs)

**Benchmarks:**
- `pipeline_single_scan` - Measures the full fetch + decode pipeline
- `client_instantiation` - Measures S3 client creation overhead
- `list_scans_parsing` - Measures scan list XML parsing
- `decompress_bzip2` - Measures bzip2 decompression performance
- `cache_get_hit` - Measures LRU cache hit performance
- `cache_get_miss` - Measures LRU cache miss performance

**Run with:**
```bash
cargo bench --package tempest-fetch
```

## Running Benchmarks

### Full Benchmark Suite

```bash
# Run all benchmarks
cargo bench

# Run specific package benchmarks
cargo bench --package tempest-decode
cargo bench --package tempest-fetch
```

### Individual Benchmarks

```bash
# Run a specific decode benchmark
cargo bench --package tempest-decode -- decode_full_volume

# Run a specific pipeline benchmark
cargo bench --package tempest-fetch -- decompress_bzip2
```

## Benchmarking Infrastructure

- **Framework:** Criterion v0.5
- **Test Fixtures:** Located in [`tempest-decode/tests/fixtures/`](tempest-decode/tests/fixtures/)
- **Mock S3:** Uses [`tempest-fetch/src/mock_s3.rs`](tempest-fetch/src/mock_s3.rs) for testing

## Baseline Measurement Process

1. Run decode benchmarks:
   ```bash
   cargo bench --package tempest-decode
   ```

2. Run pipeline benchmarks:
   ```bash
   cargo bench --package tempest-fetch
   ```

3. Update this document with the measured values from the benchmark output

## Notes

- Benchmarks use `black_box` to prevent compiler optimizations from skewing results
- Sample sizes are set to 10-20 for reliability
- Memory benchmarks track heap allocations during decoding
- p95 measurements are available through Criterion's statistics

## Current Measurement Notes

**Observed Issues:**
- `decode_full_volume` runs at ~6ns - appears to return early (likely error path)
- `pipeline_single_scan` is a stub benchmark returning immediately (black_box(()))
- `decompress_bzip2` shows realistic ~1.16µs for actual decompression work
- Memory allocation tracking not implemented in current benchmarks

**Recommendation:** Need actual pipeline benchmark implementation for meaningful p95 measurements.
