//! ABOP Command Line Interface
//!
//! This is the main entry point for the ABOP CLI application.
//! The actual CLI logic is implemented in the cli module for better organization.

mod cli;
mod commands;
mod error;
mod utils;

#[cfg(test)]
mod tests;

use crate::error::CliResult;

fn main() -> CliResult<()> {
    cli::run()
}