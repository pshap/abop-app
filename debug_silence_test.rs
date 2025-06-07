use abop_core::audio::processing::silence_detector::*;
use abop_core::audio::processing::config::*;
use abop_core::test_utils::audio::create_test_buffer_with_silence;

fn main() {
    // Create the same buffer as the failing test
    let buffer = create_test_buffer_with_silence(44100, 2, 1.0, 0.3, 0.2);
    
    println!("Buffer info:");
    println!("  Sample rate: {}", buffer.sample_rate);
    println!("  Channels: {}", buffer.channels);
    println!("  Total samples: {}", buffer.data.len());
    println!("  Duration: {:.3}s", buffer.data.len() as f32 / (buffer.sample_rate as f32 * buffer.channels as f32));
    
    // Check the actual silence region in the buffer
    let silence_start_sample = (0.3 * buffer.sample_rate as f32) as usize * buffer.channels as usize;
    let silence_end_sample = ((0.3 + 0.2) * buffer.sample_rate as f32) as usize * buffer.channels as usize;
    
    println!("Expected silence region:");
    println!("  Start sample: {}", silence_start_sample);
    println!("  End sample: {}", silence_end_sample);
    
    // Sample some values from different regions
    println!("Sample values:");
    for i in [1000, silence_start_sample, silence_start_sample + 1000, silence_end_sample, silence_end_sample + 1000] {
        if i < buffer.data.len() {
            println!("  Sample {}: {:.6}", i, buffer.data[i]);
        }
    }
    
    // Test silence detection
    let config = SilenceDetectorConfig {
        threshold_db: -60.0,
        min_duration: std::time::Duration::from_secs_f32(0.1),
        removal_mode: SilenceRemovalMode::None,
        ..Default::default()
    };
    
    let detector = SilenceDetector::new(config).unwrap();
    match detector.detect_silence_segments(&buffer) {
        Ok(segments) => {
            println!("Detected {} silence segments:", segments.len());
            for (i, segment) in segments.iter().enumerate() {
                let start_secs = segment.start as f32 / (buffer.sample_rate as f32 * buffer.channels as f32);
                println!("  Segment {}: start={}samples ({:.3}s), end={}samples ({:.3}s), duration={:.3}s", 
                    i, 
                    segment.start, 
                    start_secs,
                    segment.end,
                    segment.end as f32 / (buffer.sample_rate as f32 * buffer.channels as f32),
                    segment.duration_secs
                );
            }
        }
        Err(e) => println!("Error detecting silence: {}", e),
    }
}
