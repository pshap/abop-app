#REFINERY PROMPT

# ABOP Project Analysis & Refactoring Request

## Project Context
This is **ABOP** (Audiobook Organizing Program), an unreleased personal application in active development. You are operating on my Windows PC inside VS Code with GitHub Copilot access.

### Technical Stack
- **Language**: Rust (native)
- **GUI Framework**: Iced 13.1
- **Design System**: Translation of Material Components TypeScript to Rust
- **Reference Materials**: Available in `./material-web-clean/` directory
- **Project Structure**: `abop-gui`, `abop-core`, `abop-cli`, `material-web-clean`

### Development Philosophy
- This is pre-release software with no legacy constraints
- No need for compatibility layers or old code preservation
- Focus on modern best practices and future-proofing
- Over-engineering is encouraged to ensure maintainability and extensibility

## Task Requirements

### Step 1: File Discovery
Using PowerShell syntax, identify the 5 longest Rust files in the project:
```powershell


$files = Get-ChildItem -Path "abop-gui\src", "abop-core\src", "abop-cli\src" -Recurse -Include "*.rs" | Sort-Object Length -Descending | Select-Object -First 5; foreach ($file in $files) { Write-Host "$($file.Name) - $($file.FullName.Replace((Get-Location).Path, '.')) - $([math]::Round($file.Length/1KB,2)) KB - $((Get-Content $file.FullName | Measure-Object -Line).Lines) lines" }
```

### Step 2: Quality Analysis
Analyze files in order of size (largest first) against these **Quality Criteria**:

#### Core Rust Standards
- Rust best practices and idiomatic code
- DRYness (Don't Repeat Yourself) principles
- Proper error handling and Result/Option usage
- Memory safety and ownership patterns
- Performance optimization opportunities

#### Design & Architecture
- Good design/UI principles implementation
- Best Iced 13.1 practices and patterns
- Proper separation of concerns
- Modular, testable code structure

#### Material Components Integration
- Accurate implementation of Material Components concepts WHERE APPROPRIATE
- Consistent styling and theming approach
- Proper component abstraction and reusability
- **Note**: Do not force Material Components where they don't fit naturally

#### Documentation & Maintainability
- Accurate and concise documentation
- Clear code comments and examples
- Proper module organization
- Future-proof architecture decisions

### Step 3: Refactoring Assessment
For each file analyzed:
1. **Top 2 files**: Assess refactoring needs (low/medium/high)
2. **If medium-to-high need identified**: Create detailed refactoring plan
3. **If neither qualifies**: Move to next 3 files, then next 5
4. **If no candidates found**: Report back for guidance

### Step 4: Refactoring Plan Format
When a file needs refactoring, provide:

```markdown
## Refactoring Plan for [filename]
**Priority**: [Medium/High]
**Other file status**: [Single line note about the other analyzed file if medium+ need]

### Issues Identified
- [Specific problems found]

### Proposed Solutions
1. [Detailed step-by-step plan]
2. [Include specific code patterns/structures to implement]
3. [Performance/maintainability improvements]

### Implementation Strategy
- [Breaking changes considerations]
- [Testing approach]
- [Migration path if needed]

### Expected Benefits
- [Concrete improvements to expect]
```

## Additional Considerations
- **Online Research**: Please search for the latest Iced 13.1 documentation and Rust best practices as needed
- **Windows-Specific**: Consider Windows-specific file handling and path considerations
- **VS Code Integration**: Leverage available tooling and extensions
- **Material Design**: Reference Google's latest Material Design 3 principles

## Success Criteria
- Code is maintainable and extensible
- Follows current Rust ecosystem standards
- Implements Material Components thoughtfully
- Improves overall project architecture
- Sets foundation for future development cycles

---
*This prompt will be reused iteratively as files are cleaned up and the codebase evolves.*