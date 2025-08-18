//! ABOP Command Line Interface
//!
//! This is the main entry point for the ABOP CLI application.
//! The actual CLI logic is implemented in the cli module for better organization.

mod cli;
mod commands;
mod constants;
mod error;
mod output;
mod utils;

#[cfg(test)]
mod tests;

use crate::error::CliResult;
use clap::Parser;

fn main() -> CliResult<()> {
    // Parse args early to check if we need JSON error output
    match cli::Args::try_parse() {
        Ok(args) => {
            let json_output = args.json;
            match cli::run_with_args(args) {
                Ok(()) => Ok(()),
                Err(e) => {
                    if json_output {
                        output_json_error(&e);
                        std::process::exit(1);
                    } else {
                        Err(e)
                    }
                }
            }
        }
        Err(e) => {
            // Argument parsing errors should always be plain text
            eprintln!("{e}");
            std::process::exit(2);
        }
    }
}

fn output_json_error(error: &anyhow::Error) {
    let error_chain: Vec<String> = error.chain().map(|e| e.to_string()).collect();
    let context = if error_chain.len() > 1 {
        Some(error_chain[1..].to_vec())
    } else {
        None
    };

    let output =
        crate::output::CliOutput::error(error_chain[0].clone(), "CliError".to_string(), context);

    match output.to_json() {
        Ok(json) => {
            log::debug!("Serialized error output to JSON, size: {} bytes", json.len());
            println!("{json}");
        }
        Err(serialization_error) => {
            log::error!("Failed to serialize error output to JSON: {}", serialization_error);
            eprintln!("Error: {error}");
        }
    }
}
