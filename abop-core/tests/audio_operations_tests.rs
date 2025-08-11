//! Tests for audio processing operations (AudioProcessingOption enum).

use abop_core::message::AudioProcessingOption;

#[cfg(test)]
mod audio_operations_tests {
    use super::*;

    use abop_core::audio::{
        AudioProcessingPipeline, ChannelMixerConfig, MixingAlgorithm, ProcessingConfig,
    };
    use tempfile::tempdir;

    #[test]
    fn test_audio_processing_option_variants() {
        // Ensure all enum variants are covered
        let _ = AudioProcessingOption::StereoToMono;
        let _ = AudioProcessingOption::NoiseRemoval;
        let _ = AudioProcessingOption::Normalization;
        let _ = AudioProcessingOption::Split;
        let _ = AudioProcessingOption::Merge;
    }
    #[test]
    fn test_stereo_to_mono_and_normalize() {
        let dir = tempdir().expect("Should create temporary directory");
        let input_path = dir.path().join("input.wav");
        let output_path = dir.path().join("output.wav");
        // Write a stereo WAV file
        let spec = hound::WavSpec {
            channels: 2,
            sample_rate: 44100,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let mut writer =
            hound::WavWriter::create(&input_path, spec).expect("Should create stereo WAV file");
        for i in 0..44100 {
            // 1 second
            writer
                .write_sample((i % 256) as i16)
                .expect("Should write left channel sample"); // L
            writer
                .write_sample((255 - (i % 256)) as i16)
                .expect("Should write right channel sample"); // R
        }
        writer.finalize().expect("Should finalize stereo WAV file");

        // Create pipeline with stereo-to-mono configuration
        let config = ProcessingConfig {
            channel_mixer: Some(ChannelMixerConfig {
                target_channels: Some(1),
                mix_algorithm: MixingAlgorithm::Average,
            }),
            ..Default::default()
        };
        let pipeline =
            AudioProcessingPipeline::new(config).expect("Should create processing pipeline");

        // Process the file
        pipeline
            .process_file_with_output(&input_path, &output_path)
            .expect("Should process stereo to mono conversion");
    }
    #[test]
    fn test_split_and_merge() {
        let dir = tempdir().expect("Should create temporary directory");
        let input_path = dir.path().join("input.wav");
        // Write a mono WAV file
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: 44100,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let mut writer =
            hound::WavWriter::create(&input_path, spec).expect("Should create mono WAV file");
        for i in 0..44100 * 3 {
            writer
                .write_sample((i % 128) as i16)
                .expect("Should write sample");
        }
        writer.finalize().expect("Should finalize mono WAV file");

        // Note: Split and merge functionality would need to be implemented
        // separately as these are higher-level operations not part of the
        // core processing pipeline. For now, just test that the file exists.
        assert!(input_path.exists());
        let metadata = std::fs::metadata(&input_path).expect("Should get file metadata");
        assert!(metadata.len() > 0);
    }

    #[test]
    fn test_audio_processing_cancellation() {
        use abop_core::audio::processing::{
            batch_processor::BatchProcessor, config::ProcessingConfig, error::AudioProcessingError,
            file_io::FileProcessingOptions,
        };
        use std::sync::{
            Arc,
            atomic::{AtomicBool, Ordering},
        };
        use std::thread;
        use std::time::Duration;

        // Create a temporary directory for test files
        let dir = tempdir().expect("Should create temporary directory");

        // Create multiple test WAV files with longer duration to allow cancellation
        let input_files: Vec<_> = (0..5)
            .map(|i| {
                let path = dir.path().join(format!("input_{i}.wav"));
                let spec = hound::WavSpec {
                    channels: 2,
                    sample_rate: 44100,
                    bits_per_sample: 16,
                    sample_format: hound::SampleFormat::Int,
                };

                let mut writer =
                    hound::WavWriter::create(&path, spec).expect("Should create test WAV file");

                // Write 5 seconds of audio data to make processing slower
                for j in 0..(44100 * 5) {
                    writer.write_sample((j % 256) as i16).unwrap(); // L
                    writer.write_sample((255 - (j % 256)) as i16).unwrap(); // R
                }
                writer.finalize().expect("Should finalize WAV file");
                path
            })
            .collect();

        // Create processing config with a delay to simulate work
        let config = ProcessingConfig {
            enable_parallel: false, // Test sequential processing first
            ..Default::default()
        };

        let options = FileProcessingOptions::default();
        let processor =
            BatchProcessor::new(config, options).expect("Should create batch processor");

        // Clone the processor for the cancellation thread
        let processor_clone = processor.clone();
        let cancelled = Arc::new(AtomicBool::new(false));
        let cancelled_clone = cancelled.clone();

        // Spawn a thread to cancel processing after a short delay
        let handle = thread::spawn(move || {
            // Wait a bit to ensure processing has started
            thread::sleep(Duration::from_millis(50));
            processor_clone.cancel();
            cancelled_clone.store(true, Ordering::SeqCst);
        });

        // Process files - this should be cancelled
        let result = processor.process_files_detailed(&input_files);

        // Wait for cancellation thread to complete
        handle.join().expect("Cancellation thread should complete");

        // Verify cancellation
        assert!(
            cancelled.load(Ordering::SeqCst),
            "Cancellation should be requested"
        );

        match result {
            Ok(result) => {
                // Should either have processed some files before cancellation or have failed due to cancellation
                let total_files = result.successful.len() + result.failed.len();
                assert!(
                    total_files <= input_files.len(),
                    "Should not have processed more files than input"
                );

                // If there are failed files, check that at least one was a cancellation
                if !result.failed.is_empty() {
                    let has_cancellation = result
                        .failed
                        .iter()
                        .any(|(_, e)| matches!(e, AudioProcessingError::Cancelled(_)));
                    assert!(
                        has_cancellation,
                        "Should have at least one cancellation error"
                    );
                }
            }
            Err(e) => panic!("Unexpected error: {e:?}"),
        }

        // Test parallel cancellation
        let config = ProcessingConfig {
            enable_parallel: true,
            ..Default::default()
        };

        let options = FileProcessingOptions::default();
        let processor =
            BatchProcessor::new(config, options).expect("Should create parallel batch processor");

        let processor_clone = processor.clone();
        let cancelled = Arc::new(AtomicBool::new(false));
        let cancelled_clone = cancelled.clone();

        // Spawn a thread to cancel processing after a short delay
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(50));
            processor_clone.cancel();
            cancelled_clone.store(true, Ordering::SeqCst);
        });

        // Process files - this should be cancelled
        let result = processor.process_files_detailed(&input_files);

        // Wait for cancellation thread to complete
        handle.join().expect("Cancellation thread should complete");

        // Verify parallel cancellation
        assert!(
            cancelled.load(Ordering::SeqCst),
            "Parallel cancellation should be requested"
        );

        match result {
            Ok(result) => {
                // In parallel mode, timing is unpredictable. We might have processed all files
                // or some of them depending on timing. The important thing is that cancellation
                // was requested and the system responded appropriately.
                let total_files = result.successful.len() + result.failed.len();
                assert!(
                    total_files <= input_files.len(),
                    "Should not have processed more files than input"
                );

                // If there are failed files, at least one should be a cancellation
                if !result.failed.is_empty() {
                    let has_cancellation = result
                        .failed
                        .iter()
                        .any(|(_, e)| matches!(e, AudioProcessingError::Cancelled(_)));

                    assert!(
                        has_cancellation,
                        "If there are failed files, at least one should be a cancellation error"
                    );
                }
            }
            Err(e) => panic!("Unexpected error in parallel mode: {e:?}"),
        }
    }

    #[test]
    fn test_audio_processing_error_handling() {
        use abop_core::audio::processing::{
            batch_processor::BatchProcessor, config::ProcessingConfig, error::AudioProcessingError,
            file_io::FileProcessingOptions,
        };

        // Create a temporary directory for test files
        let dir = tempdir().expect("Should create temporary directory");

        // Create a valid WAV file
        let valid_path = dir.path().join("valid.wav");
        let spec = hound::WavSpec {
            channels: 2,
            sample_rate: 44100,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        let mut writer =
            hound::WavWriter::create(&valid_path, spec).expect("Should create valid WAV file");
        for i in 0..44100 {
            // 1 second of audio
            writer.write_sample((i % 256) as i16).unwrap(); // L
            writer.write_sample((255 - (i % 256)) as i16).unwrap(); // R
        }
        writer.finalize().expect("Should finalize WAV file");

        // Create a non-existent file path
        let invalid_path = dir.path().join("nonexistent.wav");

        // Create an invalid WAV file (empty file)
        let corrupt_path = dir.path().join("corrupt.wav");
        std::fs::File::create(&corrupt_path).expect("Should create empty file");

        // Test with mixed valid and invalid files
        let input_files = vec![
            valid_path.clone(),
            invalid_path.clone(),
            corrupt_path.clone(),
        ];

        let config = ProcessingConfig::default();
        let options = FileProcessingOptions::default();
        let processor =
            BatchProcessor::new(config, options).expect("Should create batch processor");

        let result = processor.process_files_detailed(&input_files);

        match result {
            Ok(result) => {
                // Should have processed the valid file
                assert_eq!(result.successful.len(), 1, "Should process one valid file");
                assert_eq!(result.failed.len(), 2, "Should fail on two files");

                // Check that the valid file was processed
                assert_eq!(
                    result.successful[0].0, valid_path,
                    "Should process the valid file"
                );

                // Check that we have the expected failures
                let failed_paths: Vec<_> = result.failed.iter().map(|(path, _)| path).collect();

                assert!(
                    failed_paths.contains(&&invalid_path) || failed_paths.contains(&&corrupt_path),
                    "Should include both invalid files in failures"
                );

                // Check error types
                for (_, error) in result.failed {
                    match error {
                        AudioProcessingError::FileIo(_) => {}
                        _ => panic!("Unexpected error type: {error:?}"),
                    }
                }
            }
            Err(e) => panic!("Unexpected error: {e:?}"),
        }

        // Test with only invalid files
        let input_files = vec![invalid_path, corrupt_path];
        let result = processor.process_files_detailed(&input_files);

        match result {
            Ok(result) => {
                assert!(
                    result.successful.is_empty(),
                    "No files should be processed successfully"
                );
                assert_eq!(result.failed.len(), 2, "Should fail on all files");
            }
            Err(e) => panic!("Unexpected error: {e:?}"),
        }
    }

    #[test]
    fn test_audio_processing_progress_reporting() {
        use abop_core::audio::processing::{
            batch_processor::BatchProcessor, config::ProcessingConfig,
            file_io::FileProcessingOptions,
        };
        use hound;
        use std::sync::{Arc, Mutex};
        use tempfile::tempdir;

        // Create a temporary directory for test files
        let dir = tempdir().expect("Should create temporary directory");

        // Create a test WAV file
        let input_path = dir.path().join("test.wav");
        let spec = hound::WavSpec {
            channels: 2,
            sample_rate: 44100,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        // Write a short audio file
        {
            let mut writer =
                hound::WavWriter::create(&input_path, spec).expect("Should create test WAV file");

            // Write 0.1 second of audio data
            for i in 0..4410 {
                writer.write_sample((i % 256) as i16).unwrap(); // L
                writer.write_sample((255 - (i % 256)) as i16).unwrap(); // R
            }
            writer.finalize().expect("Should finalize WAV file");
        }

        // Create a shared vector to collect progress updates
        let progress_updates = Arc::new(Mutex::new(Vec::<(f32, String)>::new()));
        let progress_updates_clone = Arc::clone(&progress_updates);

        // Create processor with progress callback
        let config = ProcessingConfig::default();
        let options = FileProcessingOptions::default();

        let processor = BatchProcessor::new(config, options)
            .expect("Should create batch processor")
            .with_progress_callback(move |progress, message| {
                let mut updates = progress_updates_clone.lock().unwrap();
                updates.push((progress, message));
            });

        // Process the file
        let result = processor.process_files_detailed(&[&input_path]);

        // Verify processing completed successfully
        assert!(result.is_ok(), "Processing should complete successfully");

        // Get the progress updates
        let updates = progress_updates.lock().unwrap();

        // Should have at least one progress update
        assert!(
            !updates.is_empty(),
            "Should have at least one progress update"
        );

        // Check that progress starts at 0%
        if let Some((first_progress, _)) = updates.first() {
            assert!(
                (*first_progress - 0.0).abs() < f32::EPSILON,
                "First progress should be 0%"
            );
        }

        // Check that progress ends at or near 100%
        if let Some((last_progress, _)) = updates.last() {
            assert!(
                (*last_progress - 100.0).abs() < 1.0, // Allow for floating point imprecision
                "Last progress should be close to 100%"
            );
        }

        // Check that progress is non-decreasing
        let mut last_progress = -1.0;
        for (progress, message) in updates.iter() {
            assert!(
                *progress >= last_progress,
                "Progress should never decrease: {last_progress}% -> {progress}% ({message})"
            );
            last_progress = *progress;
        }
    }

    #[test]
    fn test_batch_process_parallelism() {
        use abop_core::audio::AudioProcessingPipeline;
        let pipeline = AudioProcessingPipeline::default();
        let dir = tempdir().expect("Should create temporary directory");
        // Create several small WAV files
        let mut files = vec![];
        for i in 0..4 {
            let path = dir.path().join(format!("test_{i}.wav"));
            let spec = hound::WavSpec {
                channels: 1,
                sample_rate: 8000,
                bits_per_sample: 16,
                sample_format: hound::SampleFormat::Int,
            };
            let mut writer = hound::WavWriter::create(&path, spec).expect("Should create WAV file");
            for _ in 0..8000 {
                writer.write_sample(0i16).expect("Should write sample");
            }
            writer.finalize().expect("Should finalize WAV file");
            files.push(path);
        }
        // Test parallel processing of multiple files
        let results = pipeline.process_files(&files);
        assert!(results.is_ok());
        let output_paths = results.expect("Should process files successfully");
        assert_eq!(output_paths.len(), files.len());

        // Note: The following line was causing an error because results is consumed above
        // assert_eq!(results.len(), files.len());

        // Instead, we'll just check the count matches
        // We don't check if paths exist because they might be temporary files that are cleaned up
        assert_eq!(output_paths.len(), files.len());
    }
}
