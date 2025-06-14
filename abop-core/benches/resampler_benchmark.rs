//! Benchmarks for the audio resampler

use abop_core::audio::{
    AudioBuffer, SampleFormat,
    processing::{resampler::LinearResampler, traits::AudioProcessor},
};
use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use rand::{Rng, rng};

fn generate_test_audio(sample_rate: u32, channels: u16, duration_secs: f32) -> AudioBuffer<f32> {
    let samples = (sample_rate as f32 * duration_secs) as usize * channels as usize;
    let mut rng = rng();
    let data: Vec<f32> = (0..samples).map(|_| rng.random_range(-1.0..=1.0)).collect();

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

    let mut group = c.benchmark_group("resampler");

    for (src_rate, dst_rate, channels, duration) in test_cases.iter() {
        let buffer = generate_test_audio(*src_rate, *channels, *duration);
        let bytes = (buffer.data.len() * std::mem::size_of::<f32>()) as u64;

        group.throughput(Throughput::Bytes(bytes));

        // Benchmark using the public API
        group.bench_function(
            format!("public_api_{src_rate}Hz_to_{dst_rate}Hz_{channels}ch"),
            |b| {
                let mut resampler = LinearResampler::with_target_rate(*dst_rate).unwrap();
                b.iter(|| {
                    let mut buffer = buffer.clone();
                    resampler.process(&mut buffer).unwrap();
                    criterion::black_box(&buffer);
                })
            },
        );

        // Benchmark scalar implementation directly (for comparison)
        group.bench_function(
            format!(
                "direct_scalar_{src_rate}Hz_to_{dst_rate}Hz_{channels}ch"
            ),
            |b| {
                b.iter(|| {
                    let mut buffer = buffer.clone();
                    abop_core::audio::processing::resampler::LinearResampler::resample_buffer_scalar(&mut buffer, *dst_rate).unwrap();
                    criterion::black_box(&buffer);
                })
            },
        );
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
