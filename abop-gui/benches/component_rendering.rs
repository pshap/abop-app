//! Component rendering performance benchmarks
//!
//! These benchmarks measure the performance of GUI component rendering
//! with varying dataset sizes to ensure scalability.
//!
//! Run with: cargo bench --features bench

use abop_core::PlayerState;
use abop_gui::components::audio_controls::AudioControls;
use abop_gui::components::table_core::AudiobookTable;
use abop_gui::state::TableState;
use abop_gui::styling::material::MaterialTokens;
use abop_gui::test_utils::*;
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use std::collections::HashSet;

fn bench_audio_controls_rendering(c: &mut Criterion) {
    let mut group = c.benchmark_group("audio_controls_rendering");
    let tokens = MaterialTokens::default();

    for size in [10, 50, 100, 500].iter() {
        group.bench_with_input(
            BenchmarkId::new("with_selection", size),
            size,
            |b, &size| {
                let audiobooks = create_test_audiobook_batch(size, "bench");
                let selection: HashSet<String> = (0..size)
                    .step_by(10)
                    .map(|i| format!("bench_{i:03}"))
                    .collect();

                b.iter(|| {
                    let element = AudioControls::view(
                        black_box(&selection),
                        black_box(&audiobooks),
                        black_box(PlayerState::Playing),
                        black_box(&tokens),
                    );
                    black_box(element)
                });
            },
        );

        group.bench_with_input(BenchmarkId::new("no_selection", size), size, |b, &size| {
            let audiobooks = create_test_audiobook_batch(size, "bench");
            let selection = HashSet::new();

            b.iter(|| {
                let element = AudioControls::view(
                    black_box(&selection),
                    black_box(&audiobooks),
                    black_box(PlayerState::Stopped),
                    black_box(&tokens),
                );
                black_box(element)
            });
        });
    }
    group.finish();
}

fn bench_table_rendering(c: &mut Criterion) {
    let mut group = c.benchmark_group("table_rendering");
    let tokens = MaterialTokens::default();
    let table_state = TableState::default();

    for size in [10, 50, 100, 500].iter() {
        group.bench_with_input(
            BenchmarkId::new("empty_selection", size),
            size,
            |b, &size| {
                let audiobooks = create_test_audiobook_batch(size, "table");
                let selection = HashSet::new();

                b.iter(|| {
                    let element = AudiobookTable::view(
                        black_box(&audiobooks),
                        black_box(&selection),
                        black_box(&table_state),
                        black_box(&tokens),
                    );
                    black_box(element)
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("with_selection", size),
            size,
            |b, &size| {
                let audiobooks = create_test_audiobook_batch(size, "table");
                let selection: HashSet<String> =
                    (0..size.min(10)).map(|i| format!("table_{i:03}")).collect();

                b.iter(|| {
                    let element = AudiobookTable::view(
                        black_box(&audiobooks),
                        black_box(&selection),
                        black_box(&table_state),
                        black_box(&tokens),
                    );
                    black_box(element)
                });
            },
        );
    }
    group.finish();
}

fn bench_test_utilities(c: &mut Criterion) {
    let mut group = c.benchmark_group("test_utilities");

    for size in [10, 50, 100, 500, 1000].iter() {
        group.bench_with_input(
            BenchmarkId::new("create_audiobook_batch", size),
            size,
            |b, &size| {
                b.iter(|| {
                    let batch = create_test_audiobook_batch(black_box(size), "perf");
                    black_box(batch)
                });
            },
        );
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_audio_controls_rendering,
    bench_table_rendering,
    bench_test_utilities
);
criterion_main!(benches);
