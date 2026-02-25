//! Golden reference image management CLI for visual regression testing.
//!
//! This tool provides commands to manage golden reference images:
//! - `update`: Copy source images to golden directory
//! - `verify`: Compare source images against golden references
//!
//! # Running the CLI
//!
//! ```bash
//! # Update golden references
//! cargo run --package tempest-app --bin golden-cli -- update <SOURCE_DIR> <GOLDEN_DIR>
//!
//! # Verify with default 1.5% threshold
//! cargo run --package tempest-app --bin golden-cli -- verify <SOURCE_DIR> <GOLDEN_DIR>
//!
//! # Verify with custom threshold
//! cargo run --package tempest-app --bin golden-cli -- verify <SOURCE_DIR> <GOLDEN_DIR> --threshold 2.0
//! ```

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tempest_app::golden_cli::{update_golden_images, verify_images, DEFAULT_THRESHOLD};

/// CLI commands
#[derive(Subcommand)]
enum Commands {
    /// Update golden reference images
    Update {
        /// Source directory containing images to use as golden references
        #[arg(value_name = "SOURCE_DIR")]
        source_dir: PathBuf,

        /// Golden directory to store reference images
        #[arg(value_name = "GOLDEN_DIR")]
        golden_dir: PathBuf,
    },

    /// Verify current images against golden references
    Verify {
        /// Source directory containing images to verify
        #[arg(value_name = "SOURCE_DIR")]
        source_dir: PathBuf,

        /// Golden directory containing reference images
        #[arg(value_name = "GOLDEN_DIR")]
        golden_dir: PathBuf,

        /// Maximum allowed difference percentage (default: 1.5)
        #[arg(long, default_value_t = DEFAULT_THRESHOLD)]
        threshold: f64,
    },
}

/// Main CLI application
#[derive(Parser)]
#[command(name = "golden-cli")]
#[command(about = "Golden reference image management for visual regression testing", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Update {
            source_dir,
            golden_dir,
        } => {
            update_golden_images(&source_dir, &golden_dir)?;
        }

        Commands::Verify {
            source_dir,
            golden_dir,
            threshold,
        } => {
            let results = verify_images(&source_dir, &golden_dir, threshold)?;

            let failed: Vec<_> = results.iter().filter(|r| !r.passed).collect();

            if !failed.is_empty() {
                eprintln!(
                    "\nVerification FAILED: {} out of {} images differ more than {}%",
                    failed.len(),
                    results.len(),
                    threshold
                );
                std::process::exit(1);
            } else {
                println!(
                    "\nVerification PASSED: All {} images match golden references",
                    results.len()
                );
                std::process::exit(0);
            }
        }
    }

    Ok(())
}
