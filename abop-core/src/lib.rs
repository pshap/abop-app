//! ABOP Core - Core functionality for the Audiobook Organizer & Processor
//!
//! This crate contains all the business logic, data models, and shared utilities
//! for the ABOP application.

#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
#![cfg_attr(feature = "simd", feature(portable_simd))]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod app;
pub mod audio;
pub mod component;
pub mod config;
pub mod constants;
pub mod db;
pub mod error;
pub mod message;
pub mod models;
pub mod scanner;
pub mod services;
pub mod test_utils;
pub mod utils;
pub mod validation;

// Test constants module (only available in test builds)
#[cfg(test)]
pub mod test_constants;

// Re-exports from audio module
pub use audio::{
    AudioBuffer, AudioDecoder, AudioFormat, AudioPlayer, AudioProcessingPipeline, AudioStream,
    ChannelMixerConfig, MixingAlgorithm, NormalizerConfig, PlayerState, ProcessingConfig,
    ResamplerConfig, SampleFormat, SilenceDetectorConfig,
};

// Re-exports from component module
pub use component::{Component, Renderable, Updatable};

// Re-exports from config module
pub use config::Config;

// Re-exports from constants module
pub use constants::{config as config_constants};

// Re-exports from scanner module
pub use scanner::SUPPORTED_AUDIO_EXTENSIONS;

// Re-exports from error module
pub use error::{AppError, Result};

// Re-exports from message module
pub use message::AppMessage;

// Re-export AppState, ViewType, and ThemeConfig from models
pub use models::{AppData, AppState, ThemeConfig, UserPreferences, ViewType};

// Re-exports from services module
pub use services::ServiceContainer;

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::{
        audio::{
            AudioBuffer, AudioDecoder, AudioFormat, AudioPlayer, AudioProcessingPipeline,
            AudioStream, ChannelMixerConfig, MixingAlgorithm, PlayerState, ProcessingConfig,
            SampleFormat,
        },
        component::{Component, Renderable, Updatable},
        config::Config,
        constants::{config as config_constants},
        error::{AppError, Result},
        message::AppMessage,
        models::{AppData, AppState, ThemeConfig, UserPreferences, ViewType},
        services::ServiceContainer,
    };
    pub use log::{debug, error, info, trace, warn};
}
