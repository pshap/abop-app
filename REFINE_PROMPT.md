#REFINE_PROMPT

# ABOP Single File Deep Analysis

## Project Context
This is **ABOP** (Audiobook Organizing Program), an unreleased personal application in active development. You are operating on my Windows PC inside VS Code with GitHub Copilot access.

### Technical Stack
- **Language**: Rust (native)
- **GUI Framework**: Iced 13.1
- **Design System**: Translation of Material Components TypeScript to Rust
- **Reference Materials**: Available in `./material-web-clean/` directory

## Target File Analysis Request

**File to analyze**: `[FILENAME]`

Please perform a comprehensive analysis of this specific file against the following criteria:

## Analysis Framework

### 1. Current State Assessment
- **Purpose & Responsibility**: What is this file trying to accomplish?
- **Complexity Metrics**: Lines of code, function count, cyclomatic complexity estimation
- **Dependencies**: What does it import/use, and what depends on it?
- **Material Components Mapping**: Which original Material Component(s) does this represent?

### 2. Code Quality Evaluation

#### Rust Idiomatic Patterns
- [ ] Proper error handling (Result/Option usage)
- [ ] Ownership and borrowing patterns
- [ ] Iterator usage vs manual loops
- [ ] Pattern matching vs conditional logic
- [ ] Type safety and zero-cost abstractions
- [ ] Memory efficiency

#### DRY Violations & Repetition
- Identify repeated code blocks
- Similar logic patterns that could be abstracted
- Hardcoded values that should be constants/config
- Duplicate error handling patterns

#### Iced 13.1 Best Practices
- Widget composition patterns
- State management approach
- Message/update cycle efficiency
- Styling and theming implementation
- Performance considerations (unnecessary redraws, etc.)

#### Material Components Fidelity
- Accurate translation of design principles
- Proper component behavior implementation
- Appropriate adaptation for native context
- Missing or incorrectly implemented features

### 3. Architecture & Design Issues
- **Single Responsibility**: Is the file doing too much?
- **Coupling**: How tightly coupled is it to other components?
- **Extensibility**: How easy would it be to add new features?
- **Testability**: Can individual parts be unit tested?

### 4. Documentation & Maintainability
- Code comments quality and coverage
- Public API documentation
- Examples and usage patterns
- Self-documenting code vs explicit documentation

## Refactoring Recommendations

Based on your analysis, provide:

### Priority Assessment
**Refactoring Need**: [Low/Medium/High]
**Estimated Effort**: [Small/Medium/Large]
**Impact**: [Low/Medium/High]

### Specific Issues Found
```rust
// Example of problematic code pattern
[specific code snippet with issue]
```
**Problem**: [What's wrong]
**Impact**: [Why it matters]

### Detailed Refactoring Plan

#### Phase 1: Immediate Wins (Low Risk)
1. [Simple changes that don't break interfaces]
2. [Extract constants, remove duplication]
3. [Improve error handling]

#### Phase 2: Structural Changes (Medium Risk)
1. [Function extraction and reorganization]
2. [Type improvements and abstractions]
3. [State management optimization]

#### Phase 3: Architectural Changes (High Risk)
1. [Major interface changes]
2. [Component splitting or merging]
3. [Performance optimizations]

### Proposed Code Structure
```rust
// High-level outline of improved structure
mod [new_module_name] {
    // Key types and functions
}
```

### Testing Strategy
- Unit tests needed for [specific functions]
- Integration tests for [component interactions]
- Property-based tests for [complex logic]

### Migration Path
- [ ] Backward compatibility considerations
- [ ] Incremental refactoring steps
- [ ] Rollback plan if issues arise

## Success Metrics
After refactoring, the file should:
- Be easier to understand and modify
- Have better performance characteristics
- Follow Rust and Iced best practices
- Maintain or improve Material Components fidelity
- Be more testable and maintainable

## Additional Context Needed
If you need more information to complete this analysis:
- [ ] Related files to examine
- [ ] Specific Material Components reference
- [ ] Performance requirements
- [ ] Feature roadmap considerations

---
**Next Steps**: Implement the refactoring plan incrementally, testing at each phase before proceeding to the next.