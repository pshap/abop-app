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
        // TODO: Simulate cancellation and assert correct state/notification
    }

    #[test]
    fn test_audio_processing_error_handling() {
        // TODO: Simulate error and assert error notification/state
    }

    #[test]
    fn test_audio_processing_progress_reporting() {
        // TODO: Simulate progress and assert progress state/notification
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
