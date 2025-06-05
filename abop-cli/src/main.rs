//! ABOP Command Line Interface
//!
//! This is the main entry point for the ABOP CLI application.

use clap::Parser;
use log::info;

/// Command line arguments for ABOP CLI
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the audiobook library
    #[arg(short, long)]
    library: Option<std::path::PathBuf>,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    // Parse command line arguments
    let args = Args::parse();

    // Initialize logging
    env_logger::Builder::new()
        .filter_level(if args.verbose {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        })
        .init();

    info!("Starting ABOP CLI");

    // TODO: Implement CLI functionality
}
