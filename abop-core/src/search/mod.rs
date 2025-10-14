//! Search functionality for audiobooks
//!
//! This module provides search capabilities for finding audiobooks
//! based on various criteria including title, author, narrator, and more.
//!
//! # Design Rationale
//!
//! The search subsystem is designed for extensibility and performance in large audiobook libraries.
//! It uses an in-memory index for fast fuzzy and fielded search, and is decoupled from the database layer.
//! The engine supports:
//! - Fuzzy matching (via SkimMatcherV2)
//! - Fielded queries (title, author, narrator, duration, library)
//! - Real-time filtering for responsive UIs
//!
//! ## Architecture
//! - All search logic is encapsulated in the `engine` submodule.
//! - The API is designed to be thread-safe and stateless for easy integration with async and GUI workflows.
//! - Search results are returned as ranked lists, suitable for incremental display.
//!
//! ## Extensibility
//! - To add new search fields, extend the `SearchableText` struct and update the indexer.
//! - For persistent search, integrate with the database layer via a repository pattern.
//!
//! See also: `abop-core/src/search/engine.rs` for implementation details.

pub mod engine;

// Re-export commonly used types
pub use engine::SearchEngine;
