//! Test suite for the resampler module

#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests {
    use crate::audio::processing::config::ResamplerConfig;
    use crate::audio::processing::resampler::{LinearResampler, ResampleBufferFn};
    use crate::audio::processing::traits::{AudioProcessor, LatencyReporting, Validatable};
    use crate::audio::SampleFormat;
    use crate::test_utils::audio::{create_stereo_test_buffer, create_test_buffer};
    use crate::utils::casting::domain::audio::safe_duration_to_samples;

    #[cfg(feature = "simd")]
    #[test]
    fn test_simd_scalar_equivalence() {
        // Skip SIMD tests if SIMD is not enabled
        #[cfg(not(all(feature = "simd", target_arch = "x86_64")))]
        {
            println!("Skipping SIMD tests - SIMD feature not enabled or not on x86_64");
            return;
        }

        // Only run SIMD tests if we're in release mode or explicitly enabled
        if cfg!(debug_assertions) && std::env::var("RUN_SIMD_TESTS").is_err() {
            println!("Skipping SIMD tests in debug mode. Set RUN_SIMD_TESTS=1 to run them.");
            return;
        }

        use std::time::Instant;

        // Create a test buffer with a simple waveform
        let sample_rate = 44100;
        let channels = 2;
        let duration_secs = 0.1; // 100ms
        let mut buffer = create_test_buffer(sample_rate, channels, duration_secs, Some(0.5));

        // Make a copy for SIMD processing
        let mut buffer_simd = buffer.clone();

        // Target sample rate
        let target_rate = 48000;

        // Process with scalar implementation
        let scalar_start = Instant::now();
        crate::audio::processing::resampler::scalar::resample_buffer_scalar(
            &mut buffer,
            target_rate,
        )
        .expect("Scalar resampling should succeed for valid input");
        let _scalar_duration = scalar_start.elapsed();

        // Process with SIMD implementation
        crate::audio::processing::resampler::simd::resample_buffer_simd(
            &mut buffer_simd,
            target_rate,
        )
        .expect("SIMD resampling should succeed for valid input");

        // Compare results
        assert_eq!(
            buffer.data.len(),
            buffer_simd.data.len(),
            "Output buffer length mismatch for {sample_rate}Hz->{target_rate}Hz, {channels}ch"
        );
        assert_eq!(
            buffer.sample_rate, buffer_simd.sample_rate,
            "Sample rate mismatch for {sample_rate}Hz->{target_rate}Hz, {channels}ch"
        );
        assert_eq!(
            buffer.channels, buffer_simd.channels,
            "Channel count mismatch for {sample_rate}Hz->{target_rate}Hz, {channels}ch"
        );

        // Compare sample values with a small epsilon for floating-point imprecision
        let epsilon = 1e-5;
        for (i, (scalar, simd)) in buffer.data.iter().zip(buffer_simd.data.iter()).enumerate() {
            let diff = (scalar - simd).abs();
            assert!(
                diff <= epsilon,
                "Sample {i} differs by {diff} (scalar={scalar}, simd={simd})"
            );
        }

        // Additional checks for edge cases
        if !buffer.data.is_empty() {
            // Check first and last samples
            assert!((buffer.data[0] - buffer_simd.data[0]).abs() <= epsilon);
            let last = buffer.data.len() - 1;
            assert!((buffer.data[last] - buffer_simd.data[last]).abs() <= epsilon);

            // Check for NaN or infinite values
            assert!(
                buffer.data.iter().all(|&x| x.is_finite()),
                "Scalar output contains NaN or infinite values"
            );
            assert!(
                buffer_simd.data.iter().all(|&x| x.is_finite()),
                "SIMD output contains NaN or infinite values"
            );
        }
    }

    #[test]
    fn test_resampler_edge_cases() {
        // Test with very short buffers
        let mut buffer = crate::audio::AudioBuffer {
            data: vec![1.0, 0.5, -0.5, -1.0],
            format: SampleFormat::F32,
            sample_rate: 44100,
            channels: 1,
        };

        // Resample to same rate (should be no-op)
        let original_data = buffer.data.clone();
        LinearResampler::resample_buffer(&mut buffer, 44100)
            .expect("Resampling to same rate should succeed");
        assert_eq!(buffer.data, original_data);

        // Resample to half rate
        LinearResampler::resample_buffer(&mut buffer, 22050)
            .expect("Resampling to lower rate should succeed");
        assert_eq!(buffer.data.len(), 2);

        // Resample to double rate
        LinearResampler::resample_buffer(&mut buffer, 44100)
            .expect("Resampling to higher rate should succeed");
        assert_eq!(buffer.data.len(), 4);
    }

    #[test]
    fn test_resampler_empty_buffer() {
        let mut buffer = crate::audio::AudioBuffer {
            data: vec![],
            format: SampleFormat::F32,
            sample_rate: 44100,
            channels: 1,
        };

        assert!(LinearResampler::resample_buffer(&mut buffer, 48000).is_ok());
        assert!(buffer.data.is_empty());
    }

    #[test]
    fn test_resampler_creation() {
        let config = ResamplerConfig {
            target_sample_rate: Some(48000),
            ..Default::default()
        };
        let resampler = LinearResampler::new(config);
        assert!(resampler.is_ok());
    }

    #[test]
    fn test_resampler_downsample() {
        let mut buffer = create_test_buffer(44100, 1, 0.1, Some(0.5));
        let config = ResamplerConfig {
            target_sample_rate: Some(22050),
            ..Default::default()
        };

        let mut resampler = LinearResampler::new(config)
            .expect("Resampler creation should succeed with valid config");
        let result = resampler.process(&mut buffer);

        assert!(result.is_ok());
        assert_eq!(buffer.sample_rate, 22050); // Should have approximately half the samples
        let expected_samples = safe_duration_to_samples(0.1, 22050).unwrap_or(0);
        let buffer_len = i32::try_from(buffer.data.len()).unwrap_or(i32::MAX);
        let expected_i32 = i32::try_from(expected_samples).unwrap_or(i32::MAX);
        assert!((buffer_len - expected_i32).abs() <= 10);
    }

    #[test]
    fn test_resampler_upsample() {
        let mut buffer = create_test_buffer(22050, 1, 0.1, Some(0.5));
        let config = ResamplerConfig {
            target_sample_rate: Some(44100),
            ..Default::default()
        };

        let mut resampler = LinearResampler::new(config)
            .expect("Resampler creation should succeed with valid config");
        let result = resampler.process(&mut buffer);

        assert!(result.is_ok());
        assert_eq!(buffer.sample_rate, 44100); // Should have approximately double the samples
        let expected_samples = safe_duration_to_samples(0.1, 44100).unwrap_or(0);
        let buffer_len = i32::try_from(buffer.data.len()).unwrap_or(i32::MAX);
        let expected_i32 = i32::try_from(expected_samples).unwrap_or(i32::MAX);
        assert!((buffer_len - expected_i32).abs() <= 10);
    }

    #[test]
    fn test_resampler_no_change() {
        let mut buffer = create_test_buffer(44100, 1, 0.1, Some(0.5));
        let original_len = buffer.data.len();

        let config = ResamplerConfig {
            target_sample_rate: Some(44100),
            ..Default::default()
        };

        let mut resampler = LinearResampler::new(config)
            .expect("Resampler creation should succeed with valid config");
        let result = resampler.process(&mut buffer);

        assert!(result.is_ok());
        assert_eq!(buffer.sample_rate, 44100);
        assert_eq!(buffer.data.len(), original_len);
    }

    #[test]
    fn test_resampler_stereo() {
        let mut buffer = create_stereo_test_buffer(44100, 0.1);
        let config = ResamplerConfig {
            target_sample_rate: Some(48000),
            ..Default::default()
        };

        let mut resampler = LinearResampler::new(config)
            .expect("Resampler creation should succeed with valid config");
        let result = resampler.process(&mut buffer);

        assert!(result.is_ok());
        assert_eq!(buffer.sample_rate, 48000);
        assert_eq!(buffer.channels, 2);

        // Should maintain stereo channel count
        assert_eq!(buffer.data.len() % 2, 0);
    }

    #[test]
    fn test_resampler_invalid_rates() {
        let config = ResamplerConfig {
            target_sample_rate: Some(0),
            ..Default::default()
        };

        let resampler = LinearResampler::new(config);
        assert!(resampler.is_err());
    }

    #[test]
    fn test_resampler_with_target_rate() {
        let resampler = LinearResampler::with_target_rate(48000);
        assert!(resampler.is_ok());
        let created_resampler =
            resampler.expect("Resampler creation should succeed with valid target rate");
        assert_eq!(created_resampler.config.target_sample_rate, Some(48000));
    }

    #[test]
    fn test_resampler_latency() {
        let resampler = LinearResampler::default();
        assert_eq!(resampler.get_latency_samples(), 1);
    }

    #[test]
    fn test_resampler_validation() {
        let resampler = LinearResampler::default();
        assert!(resampler.validate().is_ok());
    }

    #[test]
    fn test_resample_function_types() {
        // Test default function type
        let default_fn = ResampleBufferFn::default();
        if cfg!(feature = "simd") {
            assert_eq!(default_fn, ResampleBufferFn::Simd);
        } else {
            assert_eq!(default_fn, ResampleBufferFn::Scalar);
        }

        // Test creating resampler with different function types
        let config = ResamplerConfig::default();
        let mut resampler = LinearResampler::new(config)
            .expect("Resampler creation should succeed with default config");

        #[cfg(test)]
        resampler.set_resample_fn(ResampleBufferFn::Scalar);
        #[cfg(test)]
        assert_eq!(resampler.resample_fn(), ResampleBufferFn::Scalar);
    }
}
