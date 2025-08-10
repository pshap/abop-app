# ABOP GUI Testing Documentation

This directory contains documentation for testing practices and architectures in the ABOP GUI crate.

## 📁 Documentation Files

- **[component-tests.md](./component-tests.md)** - Comprehensive guide to the modern component testing architecture

## 🧪 Testing Overview

The ABOP GUI uses a modern, well-organized testing architecture:

### Test Organization
- **Unit Tests**: Component-focused tests in `src/components/tests/`
- **Benchmarks**: Performance benchmarks in `benches/`
- **Integration Tests**: End-to-end tests in `tests/`

### Running Tests

```bash
# Run all component tests
cargo nextest run components::tests

# Run specific component tests
cargo nextest run components::tests::audio_controls

# Run benchmarks
cargo bench --features bench

# Run with output for debugging
cargo nextest run components::tests --nocapture
```

### Test Categories

1. **Functional Tests**: Basic component functionality and state handling
2. **Fuzz Tests**: Robustness testing with diverse inputs
3. **Performance Observations**: Non-assertive performance monitoring
4. **Regression Tests**: Prevention of known issues

## 🏗️ Architecture Benefits

✅ **Maintainable**: Focused test files per component  
✅ **Scalable**: Easy to add new tests and components  
✅ **Reliable**: Comprehensive edge case and regression testing  
✅ **Fast**: Parallel execution and optimized test utilities  

For detailed information, see [component-tests.md](./component-tests.md).