# Future Project Proposals

## Project #2: Component System Consolidation
**Priority:** High | **Effort:** Medium | **Impact:** Code Reduction + Maintainability

### Overview
Consolidate the fragmented component styling system that's spread across multiple modules with overlapping functionality.

### Current Issues
- Material components split across `styling/material/components/` and `components/`
- Duplicate button implementations and validation logic
- Selection components have complex builder patterns with lots of boilerplate
- Strategy pattern implementation is over-engineered for current needs

### Proposed Changes
1. **Unify Component Architecture:**
   - Merge `styling/material/components/` into `components/`
   - Create single source of truth for each component type
   - Standardize component creation patterns

2. **Simplify Builder Patterns:**
   - Replace complex validation builders with simpler factory functions
   - Consolidate selection component builders (currently ~2000 lines)
   - Remove unnecessary abstraction layers

3. **Streamline Strategy System:**
   - Replace strategy pattern with direct styling functions where appropriate
   - Keep strategies only for truly variant behavior
   - Eliminate intermediate trait abstractions

### Expected Benefits
- **Code Reduction:** ~25-30% in styling/component modules
- **Maintainability:** Single place to modify each component
- **Developer Experience:** Simpler component creation API
- **Performance:** Fewer allocations from simplified builders

### Files to Focus On
- `abop-gui/src/components/`
- `abop-gui/src/styling/material/components/`
- `abop-gui/src/styling/strategy/`

---

## Project #3: Configuration and State Management Simplification  
**Priority:** Medium | **Effort:** Medium | **Impact:** Maintainability + Performance

### Overview
Simplify the over-engineered configuration and state management system that has layers of validation, serialization, and state tracking.

### Current Issues
- Multiple config types with overlapping validation logic
- Complex state management in `UiState` with unused features
- Over-engineered validation system with multiple abstraction layers
- Router system more complex than needed for current views

### Proposed Changes
1. **Consolidate Configuration:**
   - Merge app/ui/window configs into single configuration structure
   - Simplify validation to basic checks (remove validation framework)
   - Use serde defaults instead of complex default builders

2. **Streamline State Management:**
   - Simplify `UiState` to only track actually used state
   - Remove unused state tracking features
   - Consolidate router to simple enum-based navigation

3. **Simplify Persistence:**
   - Use direct serde serialization instead of custom persistence layer
   - Remove complex error recovery mechanisms
   - Simplify file I/O to standard library approaches

### Expected Benefits
- **Code Reduction:** ~20-25% in config/state modules
- **Performance:** Faster startup (less validation overhead)
- **Maintainability:** Easier to understand and modify configuration
- **Memory Usage:** Reduced memory footprint from simpler state

### Files to Focus On
- `abop-core/src/config/`
- `abop-gui/src/state.rs`
- `abop-gui/src/router.rs`
- `abop-core/src/validation/`

---

## Implementation Notes

### Dependencies Between Projects
- **Test → Component:** Tests will make component refactoring easier to validate
- **Component → Config:** Simpler components will make state management clearer
- Could potentially combine Test + Component projects if test framework provides good coverage

### Risk Assessment
- **Test Project:** Low risk, high reward
- **Component Project:** Medium risk, very high reward  
- **Config Project:** Low-medium risk, medium reward

### Recommended Sequence
1. **Test Consolidation** (this iteration)
2. **Component System Consolidation** (next iteration)  
3. **Configuration Simplification** (future iteration)

This sequence allows each project to benefit from the previous one's improvements and maintains manageable scope per iteration.
