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

    // Generate structured error code based on error content
    let error_code = categorize_error(&error_chain[0]);
    
    let output =
        crate::output::CliOutput::error(error_chain[0].clone(), error_code, context);

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

/// Categorize error messages into structured error codes for JSON output
/// 
/// This function analyzes error messages and assigns appropriate error codes
/// to enable programmatic error handling by consumers of the JSON output.
fn categorize_error(error_message: &str) -> String {
    let error_lower = error_message.to_lowercase();
    
    match error_lower {
        msg if msg.contains("does not exist") || msg.contains("not found") => "PATH_NOT_FOUND".to_string(),
        msg if msg.contains("is a directory") || msg.contains("invalid path") => "INVALID_PATH".to_string(),
        msg if msg.contains("database") || msg.contains("sqlite") => "DATABASE_ERROR".to_string(),
        msg if msg.contains("permission") || msg.contains("access denied") => "PERMISSION_DENIED".to_string(),
        msg if msg.contains("scan") || msg.contains("scanner") => "SCAN_ERROR".to_string(),
        msg if msg.contains("audio") || msg.contains("format") => "AUDIO_ERROR".to_string(),
        msg if msg.contains("library") => "LIBRARY_ERROR".to_string(),
        msg if msg.contains("serializ") || msg.contains("json") => "SERIALIZATION_ERROR".to_string(),
        msg if msg.contains("config") => "CONFIGURATION_ERROR".to_string(),
        msg if msg.contains("network") || msg.contains("connection") => "NETWORK_ERROR".to_string(),
        _ => "UNKNOWN_ERROR".to_string(),
    }
}

