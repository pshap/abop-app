//! Helper functions for silence detection

/// Converts decibel value to linear amplitude
pub fn db_to_linear(db: f32) -> f32 {
    10.0f32.powf(db / 20.0)
}

/// Finds first non-silent sample in audio data
pub fn find_first_non_silent_sample(data: &[f32], threshold: f32) -> usize {
    data.iter().position(|&s| s.abs() > threshold).unwrap_or(0)
}

/// Finds last non-silent sample in audio data
pub fn find_last_non_silent_sample(data: &[f32], threshold: f32) -> usize {
    data.iter()
        .rposition(|&s| s.abs() > threshold)
        .unwrap_or_else(|| data.len().saturating_sub(1))
}

/// Checks if a single sample is silent
pub fn is_sample_silent(sample: f32, threshold: f32) -> bool {
    sample.abs() <= threshold
}

/// Checks if all channels in a frame are silent
pub fn is_frame_silent(frame_data: &[f32], threshold: f32) -> bool {
    frame_data
        .iter()
        .all(|&sample| is_sample_silent(sample, threshold))
}

/// Calculates the RMS level of audio data
pub fn calculate_rms_level(data: &[f32]) -> f32 {
    if data.is_empty() {
        return 0.0;
    }

    let sum_squares: f32 = data.iter().map(|&x| x * x).sum();
    (sum_squares / data.len() as f32).sqrt()
}

/// Calculates the peak level of audio data
pub fn calculate_peak_level(data: &[f32]) -> f32 {
    if data.is_empty() {
        return 0.0;
    }

    data.iter().map(|&x| x.abs()).fold(0.0, f32::max)
}

/// Converts linear amplitude to decibels
pub fn linear_to_db(linear: f32) -> f32 {
    if linear <= 0.0 {
        return -f32::INFINITY;
    }
    20.0 * linear.log10()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_db_to_linear() {
        assert!((db_to_linear(0.0) - 1.0).abs() < f32::EPSILON);
        assert!((db_to_linear(-6.0) - 0.501197).abs() < 0.001);
        assert!((db_to_linear(-20.0) - 0.1).abs() < 0.001);
        assert!((db_to_linear(-60.0) - 0.001).abs() < 0.0001);
    }

    #[test]
    fn test_linear_to_db() {
        assert!((linear_to_db(1.0) - 0.0).abs() < f32::EPSILON);
        assert!((linear_to_db(0.5) - (-6.02)).abs() < 0.01);
        assert!((linear_to_db(0.1) - (-20.0)).abs() < 0.01);
        assert!((linear_to_db(0.0) - (-f32::INFINITY)).abs() < f32::EPSILON);
    }

    #[test]
    fn test_find_first_non_silent_sample() {
        let data = vec![0.0, 0.0, 0.001, 0.0, 0.002];
        assert_eq!(find_first_non_silent_sample(&data, 0.0005), 2);
        assert_eq!(find_first_non_silent_sample(&data, 0.003), 4);
        assert_eq!(find_first_non_silent_sample(&data, 0.01), 0);
    }

    #[test]
    fn test_find_last_non_silent_sample() {
        let data = vec![0.002, 0.0, 0.001, 0.0, 0.0];
        assert_eq!(find_last_non_silent_sample(&data, 0.0005), 2);
        assert_eq!(find_last_non_silent_sample(&data, 0.003), 4);
        assert_eq!(find_last_non_silent_sample(&data, 0.01), 4);
    }

    #[test]
    fn test_is_sample_silent() {
        assert!(is_sample_silent(0.0, 0.001));
        assert!(is_sample_silent(0.0005, 0.001));
        assert!(!is_sample_silent(0.002, 0.001));
    }

    #[test]
    fn test_is_frame_silent() {
        let frame1 = vec![0.0, 0.0, 0.0];
        let frame2 = vec![0.0, 0.002, 0.0];

        assert!(is_frame_silent(&frame1, 0.001));
        assert!(!is_frame_silent(&frame2, 0.001));
    }

    #[test]
    fn test_calculate_rms_level() {
        let data = vec![1.0, -1.0, 1.0, -1.0];
        let rms = calculate_rms_level(&data);
        assert!((rms - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_calculate_peak_level() {
        let data = vec![0.5, -0.8, 0.3, -0.2];
        assert_eq!(calculate_peak_level(&data), 0.8);
    }

    #[test]
    fn test_empty_data() {
        assert_eq!(calculate_rms_level(&[]), 0.0);
        assert_eq!(calculate_peak_level(&[]), 0.0);
        assert_eq!(find_first_non_silent_sample(&[], 0.001), 0);
        assert_eq!(find_last_non_silent_sample(&[], 0.001), 0);
    }
}
