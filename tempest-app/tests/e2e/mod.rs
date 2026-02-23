//! End-to-End tests for the tempest-app crate.
//!
//! This module contains integration tests that verify the complete pipeline
//! from mock S3 server to decoded radar data and renderable output.
//!
//! # Running E2E Tests
//!
//! ```bash
//! cargo test --package tempest-app --test e2e
//! ```

mod app_harness_test;
