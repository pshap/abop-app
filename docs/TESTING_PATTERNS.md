# ABOP Testing Patterns and Guidelines

This document outlines the testing patterns and best practices for the ABOP (Audiobook Organizer & Processor) project, and provides templates for writing new tests using the centralized test utilities.

## Overview

ABOP follows a centralized test infrastructure approach to eliminate code duplication and provide consistent testing patterns across all modules. This document covers:

1. **Test Utilities Architecture**
2. **Database Testing Patterns**
3. **Component Testing Patterns** 
4. **Test Organization Guidelines**
5. **Templates for Common Test Scenarios**

## Test Utilities Architecture

### Core Principles

- **DRY (Don't Repeat Yourself)**: All test utilities are centralized to eliminate duplication
- **Consistency**: Standardized patterns across database, audio, and GUI components
- **Maintainability**: Changes to core models require minimal test updates
- **Performance**: Optimized test data creation and database setup

### Module Structure

```
abop-core/src/test_utils/
├── mod.rs          # Re-exports and common utilities
├── db.rs           # Database testing utilities
└── audio.rs        # Audio processing utilities

abop-gui/src/test_utils/
├── mod.rs          # Re-exports and legacy compatibility
└── components.rs   # GUI component testing utilities
```

## Database Testing Patterns

### Basic Setup

```rust
use abop_core::test_utils::{TestDatabase, TestDataFactory, TestAssertions};

#[test]
fn test_repository_operation() {
    // 1. Set up test database
    let db = TestDatabase::new();
    
    // 2. Create repository
    let repo = MyRepository::new(db.connection.clone());
    
    // 3. Create test data
    let library = TestDataFactory::library(Some("Test Library"), Some("/test/path"));
    
    // 4. Perform operation
    let result = repo.create(&library.name, &library.path).unwrap();
    
    // 5. Assert results
    TestAssertions::assert_library_matches(&result, "Test Library", "/test/path");
}
```

### Test Database Utilities

#### `TestDatabase`
- **`TestDatabase::new()`**: Creates fresh database with migrations
- **`TestDatabase::with_sample_libraries()`**: Pre-populated with test libraries
- **`insert_test_library()`**: Add specific test data
- **`insert_test_audiobook()`**: Add audiobook test data
- **`insert_test_progress()`**: Add progress test data

#### `TestDataFactory`
- **`library(name, path)`**: Create test library
- **`audiobook(library_id, title, path)`**: Create test audiobook
- **`progress(audiobook_id, position_seconds)`**: Create test progress
- **`audiobook_with_progress(library_id, position)`**: Create linked audiobook and progress

#### `TestAssertions`
- **`assert_library_matches(actual, expected_name, expected_path)`**: Validate library data
- **`assert_audiobook_matches(actual, library_id, title, path)`**: Validate audiobook data
- **`assert_progress_matches(actual, audiobook_id, position)`**: Validate progress data

### Database Test Template

```rust
#[cfg(test)]
mod repository_tests {
    use super::*;
    use abop_core::test_utils::{TestDatabase, TestDataFactory, TestAssertions};
    
    fn setup() -> (TestDatabase, MyRepository) {
        let db = TestDatabase::new();
        let repo = MyRepository::new(db.connection.clone());
        (db, repo)
    }
    
    #[test]
    fn test_create_success() {
        let (_db, repo) = setup();
        
        let result = repo.create("Test Name", "/test/path").unwrap();
        
        TestAssertions::assert_library_matches(&result, "Test Name", "/test/path");
    }
    
    #[test]
    fn test_create_duplicate_name() {
        let (_db, repo) = setup();
        
        // Create first item
        repo.create("Duplicate", "/path1").unwrap();
        
        // Attempt duplicate
        let result = repo.create("Duplicate", "/path2");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_find_all() {
        let (db, repo) = setup();
        
        // Pre-populate with test data
        db.insert_test_library("lib-1", "Library 1", "/path1");
        db.insert_test_library("lib-2", "Library 2", "/path2");
        
        let libraries = repo.find_all().unwrap();
        assert_eq!(libraries.len(), 2);
    }
}
```

## Component Testing Patterns

### Basic Setup

```rust
use abop_gui::test_utils::{TestDataFactory, TestStateFactory, TestAssertions};

#[test]
fn test_component_creation() {
    // 1. Create test data
    let audiobooks = TestDataFactory::audiobook_collection(5);
    let tokens = TestStateFactory::material_tokens(ThemeMode::Dark);
    let selection = TestStateFactory::selection_state(&["1", "2"]);
    
    // 2. Create component
    let element = MyComponent::view(&audiobooks, &tokens, &selection);
    
    // 3. Assert creation succeeded
    TestAssertions::assert_element_creation(element);
}
```

### Component Test Utilities

#### `TestDataFactory`
- **`audiobook(id, title)`**: Create basic test audiobook
- **`custom_audiobook(id, title, author, duration, size)`**: Create detailed audiobook
- **`audiobook_collection(count)`**: Create multiple audiobooks for bulk testing
- **`library(id, name, path)`**: Create test library
- **`progress(audiobook_id, position, completed)`**: Create test progress

#### `TestStateFactory`
- **`material_tokens(theme_mode)`**: Create Material Design tokens
- **`selection_state(selected_ids)`**: Create selection state
- **`player_states()`**: Get all player states for testing
- **`theme_modes()`**: Get all theme modes for testing

#### `TestScenarios`
- **`empty_collection()`**: Empty audiobook list
- **`single_audiobook()`**: Single audiobook scenario
- **`varied_audiobooks()`**: Audiobooks with different metadata
- **`edge_case_titles()`**: Audiobooks with problematic titles
- **`large_collection(size)`**: Large collections for performance testing

#### `TestAssertions`
- **`assert_element_creation(element)`**: Verify component creation
- **`assert_all_themes_supported(factory)`**: Test all theme modes
- **`assert_audiobook_collections_supported(factory)`**: Test various collections
- **`assert_selection_states_supported(audiobooks, factory)`**: Test selection states

### Component Test Template

```rust
#[cfg(test)]
mod component_tests {
    use super::*;
    use abop_gui::test_utils::*;
    
    #[test]
    fn test_basic_creation() {
        let audiobooks = TestDataFactory::audiobook_collection(3);
        let tokens = TestStateFactory::material_tokens(ThemeMode::Dark);
        
        let element = MyComponent::view(&audiobooks, &tokens);
        TestAssertions::assert_element_creation(element);
    }
    
    #[test]
    fn test_all_themes() {
        let audiobooks = TestDataFactory::audiobook_collection(2);
        
        TestAssertions::assert_all_themes_supported(|theme_mode| {
            let tokens = TestStateFactory::material_tokens(theme_mode);
            MyComponent::view(&audiobooks, &tokens)
        });
    }
    
    #[test]
    fn test_various_collections() {
        let tokens = TestStateFactory::material_tokens(ThemeMode::Light);
        
        TestAssertions::assert_audiobook_collections_supported(|audiobooks| {
            MyComponent::view(&audiobooks, &tokens)
        });
    }
    
    #[test]
    fn test_selection_states() {
        let audiobooks = TestDataFactory::varied_audiobooks();
        let tokens = TestStateFactory::material_tokens(ThemeMode::Dark);
        
        TestAssertions::assert_selection_states_supported(audiobooks, |books, selection| {
            MyComponent::view(&books, &tokens, &selection)
        });
    }
    
    #[test]
    fn test_edge_cases() {
        let tokens = TestStateFactory::material_tokens(ThemeMode::Light);
        
        // Test empty collection
        let element = MyComponent::view(&TestScenarios::empty_collection(), &tokens);
        TestAssertions::assert_element_creation(element);
        
        // Test edge case titles
        let element = MyComponent::view(&TestScenarios::edge_case_titles(), &tokens);
        TestAssertions::assert_element_creation(element);
    }
}
```

## Test Organization Guidelines

### File Structure

- **Unit Tests**: In the same file as the code being tested (e.g., `mod tests { ... }`)
- **Integration Tests**: In separate test files under `tests/` directory
- **Large Test Suites**: Break into focused modules (e.g., `mod repository_tests`, `mod validation_tests`)

### Naming Conventions

- **Test Functions**: `test_[operation]_[condition]_[expected_result]`
  - `test_create_library_success()`
  - `test_create_library_duplicate_name_fails()`
  - `test_find_all_empty_database_returns_empty_vec()`

- **Test Data**: Use descriptive names
  - `test_library_1`, `test_library_2` for multiple items
  - `library_with_long_name` for edge cases
  - `empty_library_collection` for boundary conditions

### Test Categories

1. **Happy Path Tests**: Normal operation scenarios
2. **Edge Case Tests**: Boundary conditions, empty inputs, maximum values
3. **Error Handling Tests**: Invalid inputs, constraint violations, system failures
4. **Performance Tests**: Large datasets, timing constraints
5. **Integration Tests**: Cross-module interactions

## Performance Testing

### Large Dataset Testing

```rust
#[test]
fn test_performance_large_collection() {
    use abop_gui::test_utils::PerformanceTestUtils;
    
    PerformanceTestUtils::test_scalability(|audiobooks| {
        let tokens = TestStateFactory::material_tokens(ThemeMode::Dark);
        MyComponent::view(&audiobooks, &tokens)
    }, 1000); // Test up to 1000 audiobooks
}
```

### Timing Measurements

```rust
#[test]
fn test_creation_performance() {
    use abop_gui::test_utils::PerformanceTestUtils;
    
    let audiobooks = TestDataFactory::audiobook_collection(100);
    let tokens = TestStateFactory::material_tokens(ThemeMode::Dark);
    
    let duration = PerformanceTestUtils::measure_creation_time(|| {
        MyComponent::view(&audiobooks, &tokens)
    });
    
    // Assert reasonable performance (adjust threshold as needed)
    assert!(duration.as_millis() < 100, "Component creation took too long: {:?}", duration);
}
```

## Migration Guidelines

### Converting Existing Tests

1. **Identify Duplicated Code**: Look for repeated test setup functions
2. **Replace with Utilities**: Use appropriate factory methods
3. **Simplify Assertions**: Use centralized assertion utilities
4. **Remove Dead Code**: Delete now-unused helper functions

### Example Migration

**Before:**
```rust
fn create_test_audiobook(id: &str, title: &str) -> Audiobook {
    let path = PathBuf::from(format!("/test/path/{title}.mp3"));
    let mut audiobook = Audiobook::new("test-library-id", &path);
    audiobook.id = id.to_string();
    audiobook.title = Some(title.to_string());
    audiobook.author = Some("Test Author".to_string());
    audiobook.duration_seconds = Some(3600);
    audiobook.size_bytes = Some(1024000);
    audiobook
}

#[test]
fn test_component() {
    let audiobook = create_test_audiobook("1", "Test Book");
    let element = MyComponent::view(&vec![audiobook]);
    // Manual assertion
    let _ = element;
}
```

**After:**
```rust
#[test]
fn test_component() {
    let audiobooks = vec![TestDataFactory::audiobook("1", "Test Book")];
    let element = MyComponent::view(&audiobooks);
    TestAssertions::assert_element_creation(element);
}
```

## Best Practices

### Do's
- ✅ Use centralized test utilities for all new tests
- ✅ Follow naming conventions consistently
- ✅ Test both happy path and error conditions
- ✅ Use descriptive test names that explain the scenario
- ✅ Group related tests into modules
- ✅ Test performance with realistic data sizes

### Don'ts
- ❌ Don't duplicate test setup code across files
- ❌ Don't hardcode test data when factories are available
- ❌ Don't skip error condition testing
- ❌ Don't write tests that depend on external state
- ❌ Don't ignore performance implications

## Troubleshooting

### Common Issues

1. **Test Database Connection Errors**
   - Ensure `TestDatabase::new()` is called for each test
   - Check that migrations run successfully

2. **Component Creation Failures**
   - Verify all required dependencies are provided
   - Check that test data matches expected formats

3. **Performance Test Failures**
   - Adjust performance thresholds based on hardware
   - Consider if the test is running in CI environment

### Getting Help

- Check existing tests in the same module for patterns
- Look at the test utilities documentation in the source code
- Run tests with `cargo test -- --nocapture` for debug output

## Future Improvements

- [ ] Add property-based testing utilities using `proptest`
- [ ] Create visual regression testing tools for GUI components
- [ ] Add automated performance benchmarking
- [ ] Integrate with fuzzing tools for robustness testing
- [ ] Create test coverage reporting and enforcement