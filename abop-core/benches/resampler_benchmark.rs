//! Benchmarks for the audio resampler

use abop_core::audio::{
    AudioBuffer, SampleFormat,
    processing::{resampler::LinearResampler, traits::AudioProcessor},
};
use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use rand::{Rng, rng};
use std::hint::black_box;

/// Audio signal types for more realistic benchmarking
#[derive(Debug, Clone, Copy)]
enum AudioSignalType {
    /// Pure random noise (original behavior)
    WhiteNoise,
    /// Sine wave at 440 Hz (musical A)
    SineWave,
    /// Mix of multiple sine waves (more realistic than pure tones)
    MultiTone,
    /// Simulated speech-like signal with varying frequency content
    SpeechLike,
}

fn generate_test_audio(sample_rate: u32, channels: u16, duration_secs: f32) -> AudioBuffer<f32> {
    generate_test_audio_with_signal(sample_rate, channels, duration_secs, AudioSignalType::MultiTone)
}

fn generate_test_audio_with_signal(
    sample_rate: u32, 
    channels: u16, 
    duration_secs: f32, 
    signal_type: AudioSignalType
) -> AudioBuffer<f32> {
    let samples_per_channel = (sample_rate as f32 * duration_secs) as usize;
    let total_samples = samples_per_channel * channels as usize;
    let mut rng = rng();
    
    let data: Vec<f32> = match signal_type {
        AudioSignalType::WhiteNoise => {
            (0..total_samples).map(|_| rng.random_range(-1.0..=1.0)).collect()
        },
        AudioSignalType::SineWave => {
            let frequency = 440.0; // A4 note
            (0..total_samples).map(|i| {
                let sample_idx = i / channels as usize;
                let time = sample_idx as f32 / sample_rate as f32;
                (2.0 * std::f32::consts::PI * frequency * time).sin() * 0.5
            }).collect()
        },
        AudioSignalType::MultiTone => {
            // Mix of fundamental and harmonics (more realistic)
            let base_freq = 220.0; // A3
            (0..total_samples).map(|i| {
                let sample_idx = i / channels as usize;
                let time = sample_idx as f32 / sample_rate as f32;
                let fundamental = (2.0 * std::f32::consts::PI * base_freq * time).sin() * 0.4;
                let harmonic2 = (2.0 * std::f32::consts::PI * base_freq * 2.0 * time).sin() * 0.2;
                let harmonic3 = (2.0 * std::f32::consts::PI * base_freq * 3.0 * time).sin() * 0.1;
                fundamental + harmonic2 + harmonic3
            }).collect()
        },
        AudioSignalType::SpeechLike => {
            // Simulate speech-like content with varying frequency and amplitude
            (0..total_samples).map(|i| {
                let sample_idx = i / channels as usize;
                let time = sample_idx as f32 / sample_rate as f32;
                
                // Modulate frequency to simulate speech formants
                let base_freq = 150.0 + 100.0 * (time * 3.0).sin();
                let formant1 = (2.0 * std::f32::consts::PI * base_freq * time).sin() * 0.3;
                let formant2 = (2.0 * std::f32::consts::PI * (base_freq * 2.5) * time).sin() * 0.2;
                
                // Add some noise for realism
                let noise = rng.random_range(-0.1..=0.1);
                
                // Amplitude modulation to simulate speech envelope
                let envelope = (time * 5.0).sin().abs();
                
                (formant1 + formant2 + noise) * envelope
            }).collect()
        },
    };

    AudioBuffer {
        data,
        format: SampleFormat::F32,
        sample_rate,
        channels,
    }
}

fn bench_resampler(c: &mut Criterion) {
    let test_cases = [
        (44100, 48000, 2, 5.0), // Stereo upsample
        (48000, 44100, 2, 5.0), // Stereo downsample
        (44100, 48000, 6, 2.0), // 5.1 surround upsample
    ];

    let signal_types = [
        AudioSignalType::MultiTone,   // Default - most realistic for music
        AudioSignalType::SpeechLike,  // For speech content testing  
        AudioSignalType::WhiteNoise,  // Worst-case scenario (high frequency content)
    ];

    let mut group = c.benchmark_group("resampler");

    for (src_rate, dst_rate, channels, duration) in test_cases.iter() {
        for signal_type in signal_types.iter() {
            let buffer = generate_test_audio_with_signal(*src_rate, *channels, *duration, *signal_type);
            let bytes = (buffer.data.len() * std::mem::size_of::<f32>()) as u64;

            group.throughput(Throughput::Bytes(bytes));

            let signal_name = match signal_type {
                AudioSignalType::WhiteNoise => "noise",
                AudioSignalType::SineWave => "sine",
                AudioSignalType::MultiTone => "multitone",
                AudioSignalType::SpeechLike => "speech",
            };

            // Benchmark using the public API
            group.bench_function(
                format!("public_api_{src_rate}Hz_to_{dst_rate}Hz_{channels}ch_{signal_name}"),
                |b| {
                    let mut resampler = LinearResampler::with_target_rate(*dst_rate).unwrap();
                    b.iter(|| {
                        let mut buffer = buffer.clone();
                        resampler.process(&mut buffer).unwrap();
                        black_box(&buffer);
                    })
                },
            );

            // Only test scalar implementation with multitone to avoid benchmark explosion
            if matches!(signal_type, AudioSignalType::MultiTone) {
                group.bench_function(
                    format!(
                        "direct_scalar_{src_rate}Hz_to_{dst_rate}Hz_{channels}ch_{signal_name}"
                    ),
                    |b| {
                        b.iter(|| {
                            let mut buffer = buffer.clone();
                            abop_core::audio::processing::resampler::LinearResampler::resample_buffer_scalar(&mut buffer, *dst_rate).unwrap();
                            black_box(&buffer);
                        })
                    },
                );
            }
        }
    }

    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .sample_size(10)  // Reduce sample size for faster benchmarks
        .warm_up_time(std::time::Duration::from_secs(1))
        .measurement_time(std::time::Duration::from_secs(5));
    targets = bench_resampler
}

criterion_main!(benches);
