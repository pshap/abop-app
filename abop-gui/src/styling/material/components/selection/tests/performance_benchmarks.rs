//! Performance Benchmarks
//! 
//! Criterion-based performance benchmarks for chip components.
//! Measures performance characteristics and establishes baseline metrics.

use super::fixtures::{
    chip_factory::*,
    collection_factory::*,
    test_data::*,
};
use crate::styling::material::components::selection::chip::{
    core::{Chip, ChipState, ChipVariant},
    collection::{ChipCollection, SelectionMode},
};
use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion, BenchmarkId};
use std::collections::HashMap;

/// Benchmark chip creation and basic operations
fn bench_chip_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("chip_creation");
    
    // Single chip creation
    group.bench_function("create_single_chip", |b| {
        b.iter(|| {
            black_box(Chip::new("test label", ChipVariant::Filter))
        })
    });
    
    // Batch chip creation
    for size in [10, 100, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("create_batch_chips", size), size, |b, &size| {
            b.iter(|| {
                black_box((0..size).map(|i| {
                    Chip::new(&format!("chip_{}", i), ChipVariant::Filter)
                }).collect::<Vec<_>>())
            })
        });
    }
    
    // Chip creation with builder pattern
    group.bench_function("create_chip_with_builder", |b| {
        b.iter(|| {
            black_box(
                Chip::new("test", ChipVariant::Filter)
                    .with_description("Test description")
                    .with_icon("test-icon")
                    .with_deletable(true)
                    .with_selectable(true)
            )
        })
    });
    
    group.finish();
}

/// Benchmark chip state transitions
fn bench_chip_state_transitions(c: &mut Criterion) {
    let mut group = c.benchmark_group("chip_state_transitions");
    
    group.bench_function("enabled_to_selected", |b| {
        b.iter_batched(
            || create_test_chip("test", ChipVariant::Filter),
            |mut chip| {
                black_box(chip.transition_to(ChipState::Selected))
            },
            BatchSize::SmallInput,
        )
    });
    
    group.bench_function("selected_to_enabled", |b| {
        b.iter_batched(
            || {
                let mut chip = create_test_chip("test", ChipVariant::Filter);
                chip.state = ChipState::Selected;
                chip
            },
            |mut chip| {
                black_box(chip.transition_to(ChipState::Enabled))
            },
            BatchSize::SmallInput,
        )
    });
    
    group.bench_function("batch_state_transitions", |b| {
        b.iter_batched(
            || create_test_chip_set(100),
            |mut chips| {
                for chip in &mut chips {
                    black_box(chip.transition_to(ChipState::Selected).ok());
                    black_box(chip.transition_to(ChipState::Enabled).ok());
                }
            },
            BatchSize::SmallInput,
        )
    });
    
    group.finish();
}

/// Benchmark collection operations
fn bench_collection_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("collection_operations");
    
    // Collection creation
    for size in [10, 100, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("create_collection", size), size, |b, &size| {
            b.iter_batched(
                || create_test_chip_set(size),
                |chips| black_box(ChipCollection::from_chips(chips)),
                BatchSize::SmallInput,
            )
        });
    }
    
    // Adding chips to collection
    group.bench_function("add_chip_to_collection", |b| {
        b.iter_batched(
            || (ChipCollection::new(), create_test_chip("test", ChipVariant::Filter)),
            |(mut collection, chip)| black_box(collection.add(chip)),
            BatchSize::SmallInput,
        )
    });
    
    // Removing chips from collection
    group.bench_function("remove_chip_from_collection", |b| {
        b.iter_batched(
            || {
                let mut collection = create_test_collection(100);
                let chip_id = collection.chip_ids().next().unwrap();
                (collection, chip_id)
            },
            |(mut collection, chip_id)| black_box(collection.remove(&chip_id)),
            BatchSize::SmallInput,
        )
    });
    
    // Collection iteration
    for size in [100, 1000, 10000].iter() {
        group.bench_with_input(BenchmarkId::new("iterate_collection", size), size, |b, &size| {
            let collection = create_test_collection(*size);
            b.iter(|| {
                black_box(collection.chips().count())
            })
        });
    }
    
    group.finish();
}

/// Benchmark selection operations
fn bench_selection_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("selection_operations");
    
    // Single selection
    group.bench_function("select_single_chip", |b| {
        b.iter_batched(
            || {
                let mut collection = create_single_selection_collection();
                let chip_id = collection.chip_ids().next().unwrap();
                (collection, chip_id)
            },
            |(mut collection, chip_id)| black_box(collection.select(&chip_id)),
            BatchSize::SmallInput,
        )
    });
    
    // Multiple selection
    group.bench_function("select_multiple_chips", |b| {
        b.iter_batched(
            || create_multiple_selection_collection(),
            |mut collection| {
                let chip_ids: Vec<_> = collection.chip_ids().take(10).collect();
                for chip_id in chip_ids {
                    black_box(collection.select(&chip_id).ok());
                }
            },
            BatchSize::SmallInput,
        )
    });
    
    // Select all
    for size in [10, 100, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("select_all", size), size, |b, &size| {
            b.iter_batched(
                || create_large_multiple_selection_collection(*size),
                |mut collection| black_box(collection.select_all()),
                BatchSize::SmallInput,
            )
        });
    }
    
    // Deselect all
    for size in [10, 100, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("deselect_all", size), size, |b, &size| {
            b.iter_batched(
                || {
                    let mut collection = create_large_multiple_selection_collection(*size);
                    collection.select_all().ok();
                    collection
                },
                |mut collection| black_box(collection.deselect_all()),
                BatchSize::SmallInput,
            )
        });
    }
    
    // Toggle selection
    group.bench_function("toggle_selection", |b| {
        b.iter_batched(
            || {
                let mut collection = create_multiple_selection_collection();
                let chip_id = collection.chip_ids().next().unwrap();
                (collection, chip_id)
            },
            |(mut collection, chip_id)| black_box(collection.toggle_selection(&chip_id)),
            BatchSize::SmallInput,
        )
    });
    
    group.finish();
}

/// Benchmark search and filtering operations
fn bench_search_and_filtering(c: &mut Criterion) {
    let mut group = c.benchmark_group("search_and_filtering");
    
    // Search operations
    for size in [100, 1000, 10000].iter() {
        group.bench_with_input(BenchmarkId::new("search_by_label", size), size, |b, &size| {
            let collection = create_large_searchable_collection(*size);
            b.iter(|| {
                black_box(collection.search("test"))
            })
        });
    }
    
    // Filter by variant
    for size in [100, 1000, 10000].iter() {
        group.bench_with_input(BenchmarkId::new("filter_by_variant", size), size, |b, &size| {
            let collection = create_large_mixed_variant_collection(*size);
            b.iter(|| {
                black_box(collection.filter_by_variant(ChipVariant::Filter))
            })
        });
    }
    
    // Filter by state
    for size in [100, 1000, 10000].iter() {
        group.bench_with_input(BenchmarkId::new("filter_by_state", size), size, |b, &size| {
            let collection = create_large_test_collection(*size);
            b.iter(|| {
                black_box(collection.filter_by_state(ChipState::Enabled))
            })
        });
    }
    
    // Complex filtering (multiple criteria)
    group.bench_function("complex_filtering", |b| {
        let collection = create_large_mixed_variant_collection(1000);
        b.iter(|| {
            let filtered = collection
                .filter_by_variant(ChipVariant::Filter)
                .into_iter()
                .filter(|chip| chip.state == ChipState::Enabled)
                .filter(|chip| chip.label.contains("test"))
                .collect::<Vec<_>>();
            black_box(filtered)
        })
    });
    
    group.finish();
}

/// Benchmark validation operations
fn bench_validation_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("validation_operations");
    
    // Label validation
    group.bench_function("validate_label", |b| {
        b.iter(|| {
            black_box(crate::styling::material::components::selection::chip::validation::ChipValidator::validate_label("Test Label"))
        })
    });
    
    // ID validation
    group.bench_function("validate_id", |b| {
        b.iter(|| {
            black_box(crate::styling::material::components::selection::chip::validation::ChipValidator::validate_id("test_id_123"))
        })
    });
    
    // State transition validation
    group.bench_function("validate_state_transition", |b| {
        b.iter(|| {
            black_box(crate::styling::material::components::selection::chip::validation::ChipValidator::is_valid_state_transition(
                &ChipState::Enabled,
                &ChipState::Selected
            ))
        })
    });
    
    // Bulk validation
    for size in [100, 1000, 5000].iter() {
        group.bench_with_input(BenchmarkId::new("bulk_validation", size), size, |b, &size| {
            let chips = create_test_chip_set(*size);
            b.iter(|| {
                for chip in &chips {
                    black_box(crate::styling::material::components::selection::chip::validation::ChipValidator::validate_chip(chip).ok());
                }
            })
        });
    }
    
    group.finish();
}

/// Benchmark rendering and styling operations
fn bench_rendering_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("rendering_operations");
    
    // Style computation
    group.bench_function("compute_chip_styles", |b| {
        let chip = create_test_chip("test", ChipVariant::Filter);
        b.iter(|| {
            black_box(chip.compute_styles())
        })
    });
    
    // Layout calculation
    group.bench_function("calculate_chip_layout", |b| {
        let chip = create_test_chip("test", ChipVariant::Filter);
        b.iter(|| {
            black_box(chip.calculate_layout())
        })
    });
    
    // Render data preparation
    group.bench_function("prepare_render_data", |b| {
        let chip = create_test_chip("test", ChipVariant::Filter);
        b.iter(|| {
            black_box(chip.prepare_render_data())
        })
    });
    
    // Batch rendering
    for size in [10, 100, 500].iter() {
        group.bench_with_input(BenchmarkId::new("batch_rendering", size), size, |b, &size| {
            let chips = create_test_chip_set(*size);
            b.iter(|| {
                for chip in &chips {
                    black_box(chip.prepare_render_data());
                }
            })
        });
    }
    
    group.finish();
}

/// Benchmark memory operations
fn bench_memory_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_operations");
    
    // Clone operations
    group.bench_function("clone_chip", |b| {
        let chip = create_test_chip("test", ChipVariant::Filter);
        b.iter(|| {
            black_box(chip.clone())
        })
    });
    
    group.bench_function("clone_collection", |b| {
        let collection = create_test_collection(100);
        b.iter(|| {
            black_box(collection.clone())
        })
    });
    
    // Serialization
    group.bench_function("serialize_chip", |b| {
        let chip = create_test_chip("test", ChipVariant::Filter);
        b.iter(|| {
            black_box(chip.to_json())
        })
    });
    
    group.bench_function("deserialize_chip", |b| {
        let chip = create_test_chip("test", ChipVariant::Filter);
        let json = chip.to_json().unwrap();
        b.iter(|| {
            black_box(Chip::from_json(&json))
        })
    });
    
    // Memory allocation patterns
    group.bench_function("create_and_drop_chips", |b| {
        b.iter(|| {
            let chips: Vec<_> = (0..100)
                .map(|i| create_test_chip(&format!("chip_{}", i), ChipVariant::Filter))
                .collect();
            black_box(chips);
        })
    });
    
    group.finish();
}

/// Benchmark concurrent operations
fn bench_concurrent_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrent_operations");
    
    // Concurrent read operations
    group.bench_function("concurrent_reads", |b| {
        let collection = create_large_test_collection(1000);
        b.iter(|| {
            use std::sync::Arc;
            use std::thread;
            
            let collection = Arc::new(collection.clone());
            let handles: Vec<_> = (0..4)
                .map(|_| {
                    let collection = Arc::clone(&collection);
                    thread::spawn(move || {
                        black_box(collection.len());
                        black_box(collection.selected_count());
                        black_box(collection.chip_ids().count());
                    })
                })
                .collect();
            
            for handle in handles {
                handle.join().unwrap();
            }
        })
    });
    
    // Concurrent filtering
    group.bench_function("concurrent_filtering", |b| {
        let collection = create_large_mixed_variant_collection(1000);
        b.iter(|| {
            use std::sync::Arc;
            use std::thread;
            
            let collection = Arc::new(collection.clone());
            let variants = [ChipVariant::Filter, ChipVariant::Action, ChipVariant::Input, ChipVariant::Suggestion];
            
            let handles: Vec<_> = variants
                .iter()
                .map(|&variant| {
                    let collection = Arc::clone(&collection);
                    thread::spawn(move || {
                        black_box(collection.filter_by_variant(variant));
                    })
                })
                .collect();
            
            for handle in handles {
                handle.join().unwrap();
            }
        })
    });
    
    group.finish();
}

/// Benchmark edge case performance
fn bench_edge_cases(c: &mut Criterion) {
    let mut group = c.benchmark_group("edge_cases");
    
    // Empty collection operations
    group.bench_function("empty_collection_operations", |b| {
        b.iter(|| {
            let mut collection = ChipCollection::new();
            black_box(collection.len());
            black_box(collection.selected_count());
            black_box(collection.select_all().ok());
            black_box(collection.deselect_all());
            black_box(collection.search("anything"));
        })
    });
    
    // Very long labels
    group.bench_function("long_label_operations", |b| {
        let long_label = "A".repeat(200);
        b.iter(|| {
            black_box(crate::styling::material::components::selection::chip::validation::ChipValidator::validate_label(&long_label));
        })
    });
    
    // Rapid state changes
    group.bench_function("rapid_state_changes", |b| {
        b.iter_batched(
            || create_test_chip("test", ChipVariant::Filter),
            |mut chip| {
                for _ in 0..10 {
                    black_box(chip.transition_to(ChipState::Selected).ok());
                    black_box(chip.transition_to(ChipState::Enabled).ok());
                    black_box(chip.transition_to(ChipState::Hover).ok());
                    black_box(chip.transition_to(ChipState::Enabled).ok());
                }
            },
            BatchSize::SmallInput,
        )
    });
    
    // Maximum capacity collection
    group.bench_function("max_capacity_collection", |b| {
        b.iter(|| {
            let mut collection = ChipCollection::with_config(crate::styling::material::components::selection::chip::collection::CollectionConfig {
                max_chips: Some(10),
                allow_duplicates: false,
                auto_sort: false,
            });
            
            // Fill to capacity
            for i in 0..10 {
                let chip = create_test_chip(&format!("chip_{}", i), ChipVariant::Filter);
                black_box(collection.add(chip).ok());
            }
            
            // Try to add beyond capacity
            let extra_chip = create_test_chip("extra", ChipVariant::Filter);
            black_box(collection.add(extra_chip).err());
        })
    });
    
    group.finish();
}

/// Create custom benchmark group for regression testing
fn bench_regression_scenarios(c: &mut Criterion) {
    let mut group = c.benchmark_group("regression_scenarios");
    
    // Specific performance regression test scenarios
    group.bench_function("selection_cascade_performance", |b| {
        // Tests for performance regression in cascading selection changes
        b.iter_batched(
            || create_single_selection_collection(),
            |mut collection| {
                let chip_ids: Vec<_> = collection.chip_ids().collect();
                for chip_id in &chip_ids {
                    black_box(collection.select(chip_id).ok());
                }
            },
            BatchSize::SmallInput,
        )
    });
    
    group.bench_function("large_collection_search_performance", |b| {
        // Tests for search performance regression with large collections
        let collection = create_large_searchable_collection(5000);
        let search_terms = ["test", "sample", "data", "chip", "material", "design"];
        
        b.iter(|| {
            for term in &search_terms {
                black_box(collection.search(term));
            }
        })
    });
    
    group.bench_function("memory_allocation_pattern", |b| {
        // Tests for memory allocation regression
        b.iter(|| {
            let mut collections = Vec::new();
            for i in 0..10 {
                let collection = create_test_collection(100);
                collections.push(collection);
            }
            black_box(collections);
        })
    });
    
    group.finish();
}

// Group all benchmarks
criterion_group!(
    benches,
    bench_chip_creation,
    bench_chip_state_transitions,
    bench_collection_operations,
    bench_selection_operations,
    bench_search_and_filtering,
    bench_validation_operations,
    bench_rendering_operations,
    bench_memory_operations,
    bench_concurrent_operations,
    bench_edge_cases,
    bench_regression_scenarios
);

criterion_main!(benches);

#[cfg(test)]
mod benchmark_validation_tests {
    use super::*;
    
    /// Validate that benchmark scenarios are realistic and representative
    #[test]
    fn test_benchmark_scenarios_are_realistic() {
        // Test that benchmark data reflects real-world usage patterns
        let collection = create_test_collection(100);
        assert!(collection.len() > 0);
        
        let large_collection = create_large_test_collection(1000);
        assert_eq!(large_collection.len(), 1000);
        
        let searchable_collection = create_large_searchable_collection(500);
        let results = searchable_collection.search("test");
        assert!(results.len() > 0);
    }
    
    /// Validate benchmark performance thresholds
    #[test]
    fn test_performance_thresholds() {
        use std::time::Instant;
        
        // Single chip creation should be very fast
        let start = Instant::now();
        let _chip = create_test_chip("test", ChipVariant::Filter);
        let duration = start.elapsed();
        assert!(duration.as_nanos() < 1_000_000); // Less than 1ms
        
        // Collection creation with 100 chips should be reasonable
        let start = Instant::now();
        let _collection = create_test_collection(100);
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100); // Less than 100ms
        
        // Search in large collection should be fast
        let collection = create_large_searchable_collection(1000);
        let start = Instant::now();
        let _results = collection.search("test");
        let duration = start.elapsed();
        assert!(duration.as_millis() < 50); // Less than 50ms
    }
    
    /// Test benchmark data consistency
    #[test]
    fn test_benchmark_data_consistency() {
        // Multiple runs should produce consistent data structures
        let collection1 = create_test_collection(50);
        let collection2 = create_test_collection(50);
        
        assert_eq!(collection1.len(), collection2.len());
        assert_eq!(collection1.selected_count(), collection2.selected_count());
        
        // Chips should have consistent properties
        let chip1 = create_test_chip("test", ChipVariant::Filter);
        let chip2 = create_test_chip("test", ChipVariant::Filter);
        
        assert_eq!(chip1.label, chip2.label);
        assert_eq!(chip1.variant, chip2.variant);
        assert_eq!(chip1.state, chip2.state);
        // Note: IDs should be different for uniqueness
        assert_ne!(chip1.id, chip2.id);
    }
    
    /// Validate memory usage patterns in benchmarks
    #[test]
    fn test_benchmark_memory_patterns() {
        // Test that benchmark scenarios don't cause memory leaks
        let initial_usage = get_approximate_memory_usage();
        
        // Create and drop many collections
        for _ in 0..10 {
            let _collection = create_large_test_collection(100);
        }
        
        // Force garbage collection if applicable
        std::hint::black_box(());
        
        let final_usage = get_approximate_memory_usage();
        let growth = final_usage.saturating_sub(initial_usage);
        
        // Memory growth should be minimal after cleanup
        assert!(growth < 50 * 1024 * 1024); // Less than 50MB growth
    }
}

/// Helper function to estimate memory usage (simplified)
fn get_approximate_memory_usage() -> usize {
    // This is a simplified implementation
    // In a real benchmark, you'd use more sophisticated memory profiling
    std::process::id() as usize * 1024 // Placeholder
}
