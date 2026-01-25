//! Database module for ABOP
//!
//! This module provides database functionality for storing and retrieving
//! audiobook metadata and library information.

pub mod connection;
pub mod datetime_serde;
pub mod error;
pub mod health;
pub mod helpers;
pub mod mappers;
pub mod migrations;
pub mod operations;
pub mod repositories;
pub mod retry;
pub mod statistics;

// New, focused submodules extracted from the original monolithic mod.rs
mod app_database;
mod convenience;
mod facade;
mod legacy_crud;
mod schema;

// Public re-exports to preserve the public API surface
pub use self::connection::{ConnectionConfig, EnhancedConnection};
pub use self::error::{DatabaseError, DbResult};
pub use self::health::ConnectionHealth;
pub use self::helpers::{
    DatabaseHelpers, PoolHelper, execute_bulk_insert, parse_datetime_string, with_connection,
    with_connection_mut,
};
pub use self::mappers::{AudiobookColumnIndices, RowMappers, SqlQueries};
pub use self::migrations::{Migration, MigrationManager, MigrationResult};
pub use self::operations::DatabaseOperations;
pub use self::repositories::{
    AudiobookRepository, LibraryRepository, ProgressRepository, Repository, RepositoryManager,
};
pub use self::retry::{RetryExecutor, RetryPolicy};
pub use self::statistics::ConnectionStats;

// Re-export core facade types
pub use self::facade::{Database, PoolConfig};
