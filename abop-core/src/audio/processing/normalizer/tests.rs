//! Test suite for the normalizer module

#[cfg(test)]
mod tests {
    use crate::audio::AudioNormalizer;
    use crate::audio::SampleFormat;
    use crate::audio::processing::config::{NormalizationAlgorithm, NormalizerConfig};
    use crate::audio::processing::traits::core::AudioProcessor;
    use crate::test_utils::audio::create_test_buffer;

    #[test]
    fn test_normalizer_creation() {
        let config = NormalizerConfig {
            target_loudness: -16.0,
            ..Default::default()
        };
        let normalizer = AudioNormalizer::new(config);
        assert!(normalizer.is_ok());
    }

    #[test]
    fn test_peak_normalization() {
        let mut buffer = create_test_buffer(44100, 1, 0.1, Some(0.1)); // Quiet signal
        let config = NormalizerConfig {
            algorithm: NormalizationAlgorithm::Peak,
            target_loudness: -6.0, // Target -6dB
            headroom_db: 1.0,      // 1dB headroom
            ..Default::default()
        };

        let mut normalizer = AudioNormalizer::new(config).unwrap();
        let result = normalizer.process(&mut buffer);

        assert!(result.is_ok());

        // Check that peak is now close to target with headroom
        let max_sample = buffer.data.iter().fold(0.0f32, |max, &s| max.max(s.abs()));
        let peak_db = 20.0 * max_sample.log10();

        // Should be close to -7dB (-6dB target - 1dB headroom)
        assert!((peak_db - (-7.0)).abs() < 1.0);
    }

    #[test]
    fn test_rms_normalization() {
        let mut buffer = create_test_buffer(44100, 1, 0.1, Some(0.1)); // Quiet signal
        let config = crate::audio::processing::config::NormalizerConfig {
            algorithm: NormalizationAlgorithm::Rms,
            target_loudness: -12.0,
            headroom_db: 2.0,
            ..Default::default()
        };

        let mut normalizer = AudioNormalizer::new(config).unwrap();
        let result = normalizer.process(&mut buffer);

        assert!(result.is_ok());

        // Check that signal was amplified
        let max_sample = buffer.data.iter().fold(0.0f32, |max, &s| max.max(s.abs()));
        assert!(max_sample > 0.1); // Should be louder than original
    }

    #[test]
    fn test_empty_buffer_unchanged() {
        let mut buffer = crate::audio::AudioBuffer {
            data: Vec::new(),
            format: SampleFormat::F32,
            sample_rate: 44100,
            channels: 1,
        };

        let config = NormalizerConfig::default();
        let mut normalizer = AudioNormalizer::new(config).unwrap();
        let result = normalizer.process(&mut buffer);

        assert!(result.is_ok());

        // Buffer should remain empty
        assert!(buffer.data.is_empty());
    }

    #[test]
    fn test_with_peak_target() {
        let normalizer = AudioNormalizer::with_peak_target(-3.0);
        assert!(normalizer.is_ok());
        let normalizer = normalizer.unwrap();
        assert_eq!(normalizer.config.target_loudness, -3.0);
        assert!(matches!(
            normalizer.config.algorithm,
            NormalizationAlgorithm::Peak
        ));
    }

    #[test]
    fn test_with_rms_target() {
        let normalizer = AudioNormalizer::with_rms_target(-12.0);
        assert!(normalizer.is_ok());
        let normalizer = normalizer.unwrap();
        assert_eq!(normalizer.config.target_loudness, -12.0);
        assert!(matches!(
            normalizer.config.algorithm,
            NormalizationAlgorithm::Rms
        ));
    }

    #[test]
    fn test_calculate_peak_db() {
        let buffer = create_test_buffer(44100, 1, 0.1, Some(0.5));
        let normalizer = AudioNormalizer::default();

        let peak_db = normalizer.calculate_peak_db(&buffer);
        let expected_db = 20.0 * 0.5f32.log10(); // About -6dB

        assert!((peak_db - expected_db).abs() < 0.1);
    }

    #[test]
    fn test_calculate_rms_db() {
        let buffer = create_test_buffer(44100, 1, 0.1, Some(0.5));
        let normalizer = AudioNormalizer::default();

        let rms_db = normalizer.calculate_rms_db(&buffer);
        // RMS of a sine wave is amplitude / sqrt(2)
        let expected_rms = 0.5 / (2.0_f32).sqrt();
        let expected_db = 20.0 * expected_rms.log10();

        assert!((rms_db - expected_db).abs() < 0.1);
    }

    #[test]
    fn test_limiter() {
        let mut buffer = create_test_buffer(44100, 1, 0.1, Some(1.5)); // Clipped signal
        let normalizer = AudioNormalizer::default();

        let result = normalizer.apply_limiter(&mut buffer, 0.9);
        assert!(result.is_ok());

        // Check that no sample exceeds threshold
        let max_sample = buffer.data.iter().fold(0.0f32, |max, &s| max.max(s.abs()));
        assert!(max_sample <= 0.9);
    }

    #[test]
    fn test_empty_buffer() {
        let mut buffer = crate::audio::AudioBuffer {
            data: Vec::new(),
            format: SampleFormat::F32,
            sample_rate: 44100,
            channels: 1,
        };

        let mut normalizer = AudioNormalizer::default();
        let result = normalizer.process(&mut buffer);

        assert!(result.is_ok());
        assert!(buffer.data.is_empty());
    }

    #[test]
    fn test_normalizer_latency() {
        let normalizer = AudioNormalizer::default();
        assert_eq!(normalizer.get_latency_samples(), 0);
    }

    #[test]
    fn test_normalizer_validation() {
        let normalizer = AudioNormalizer::default();
        assert!(normalizer.validate().is_ok());
    }
}
