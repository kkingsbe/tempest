//! End-to-End tests for the tempest-fetch crate.
//!
//! This module contains integration tests that verify the complete fetch pipeline
//! from mock S3 server to data retrieval and decompression.
//!
//! # Running E2E Tests
//!
//! ```bash
//! cargo test --package tempest-fetch --test e2e
//! ```

mod fetch_pipeline_test;
