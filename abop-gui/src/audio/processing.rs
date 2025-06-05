//! Audio processing functionality for conversions and transformations

use abop_core::audio::processing::file_io::{AudioFileProcessor, FileProcessingOptions};
use abop_core::{
    AudioProcessingPipeline, ChannelMixerConfig, MixingAlgorithm, ProcessingConfig,
    models::Audiobook,
};
use std::collections::HashSet;
use std::path::PathBuf;

/// Async function to convert selected audiobooks to mono
///
/// # Errors
///
/// Returns an error if:
/// - No audiobooks are selected for conversion
/// - Audio processing pipeline creation fails
/// - File processing fails for any selected audiobook
/// - Output file creation fails
pub async fn convert_selected_to_mono(
    selected_ids: HashSet<String>,
    audiobooks: Vec<Audiobook>,
) -> Result<String, String> {
    if selected_ids.is_empty() {
        return Err("No audiobooks selected for conversion".to_string());
    }

    // Create audio processing pipeline with mono conversion configuration
    let config = ProcessingConfig {
        channel_mixer: Some(ChannelMixerConfig {
            target_channels: Some(1), // Convert to mono
            mix_algorithm: MixingAlgorithm::Average,
        }),
        ..Default::default()
    };
    let pipeline = AudioProcessingPipeline::new(config)
        .map_err(|e| format!("Failed to create audio pipeline: {e}"))?;
    let options = FileProcessingOptions::default();
    let mut processor = AudioFileProcessor::new(pipeline, options);

    // Filter to get selected audiobooks
    let selected_audiobooks: Vec<_> = audiobooks
        .iter()
        .filter(|audiobook| selected_ids.contains(&audiobook.id))
        .collect();

    if selected_audiobooks.is_empty() {
        return Err("Selected audiobooks not found".to_string());
    }

    let mut processed_count = 0;
    let mut failed_count = 0;

    // Process each selected audiobook
    for audiobook in selected_audiobooks {
        let input_path = PathBuf::from(&audiobook.path);

        // Generate output path (same directory, with "_mono" suffix)
        let output_path = input_path.parent().map_or_else(
            || {
                let title = audiobook.title.as_deref().unwrap_or("unknown");
                PathBuf::from(format!("{title}_mono.wav"))
            },
            |parent| {
                let stem = input_path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown");
                let extension = input_path
                    .extension()
                    .and_then(|s| s.to_str())
                    .unwrap_or("wav");
                parent.join(format!("{stem}_mono.{extension}"))
            },
        );

        // Perform the conversion using the file processor
        match processor.process_file_with_output(&input_path, &output_path) {
            Ok(()) => {
                let title = audiobook.title.as_deref().unwrap_or("Unknown");
                log::info!("Successfully converted '{title}' to mono");
                processed_count += 1;
            }
            Err(e) => {
                let title = audiobook.title.as_deref().unwrap_or("Unknown");
                log::error!("Failed to convert '{title}': {e}");
                failed_count += 1;
            }
        }
    }

    // Return result summary
    if failed_count == 0 {
        Ok(format!(
            "Successfully converted {processed_count} audiobook(s) to mono"
        ))
    } else if processed_count == 0 {
        Err(format!("Failed to convert all {failed_count} audiobook(s)"))
    } else {
        Ok(format!(
            "Converted {processed_count} audiobook(s) to mono ({failed_count} failed)"
        ))
    }
}
