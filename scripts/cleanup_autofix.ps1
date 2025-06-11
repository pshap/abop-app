#!/usr/bin/env pwsh
# Automated Cleanup Script for ABOP Codebase
# Run after major refactoring to clean up common clippy warnings

Write-Host "🧹 Starting ABOP Codebase Cleanup..." -ForegroundColor Green

# Phase 1: Auto-fixable issues
Write-Host "`n📋 Phase 1: Auto-fixing clippy warnings..." -ForegroundColor Yellow

# Fix all auto-fixable clippy warnings
Write-Host "  ⚡ Running cargo clippy --fix for all auto-fixable issues..."
cargo clippy --fix --allow-dirty --allow-staged --all-targets --all-features

# Fix specific components
Write-Host "  🔧 Fixing specific test files..."
cargo clippy --fix --lib -p abop-core --tests --allow-dirty --allow-staged
cargo clippy --fix --lib -p abop-gui --tests --allow-dirty --allow-staged

# Fix examples
Write-Host "  📚 Fixing examples..."
cargo clippy --fix --example "debug_path_matching" --allow-dirty --allow-staged
cargo clippy --fix --example "validate_centralization" --allow-dirty --allow-staged
cargo clippy --fix --test "debug_config_format" --allow-dirty --allow-staged
cargo clippy --fix --test "scanner_integration_tests" --allow-dirty --allow-staged

# Phase 2: Check remaining warnings
Write-Host "`n📊 Phase 2: Checking remaining warnings..." -ForegroundColor Yellow
Write-Host "  📈 Running final clippy check..."
$clippy_output = cargo clippy --all-targets --all-features 2>&1
$warning_count = ($clippy_output | Select-String "warning:").Count

Write-Host "`n📈 Results:" -ForegroundColor Cyan
Write-Host "  Remaining warnings: $warning_count" -ForegroundColor White

if ($warning_count -gt 0) {
    Write-Host "  ⚠️  Manual fixes still needed. See CLEANUP_TRACKING.md for details." -ForegroundColor Yellow
    Write-Host "  📝 Generating updated clippy warnings file..."
    $clippy_output | Out-File -FilePath "clippy_warnings_after_autofix.txt" -Encoding UTF8
} else {
    Write-Host "  ✅ All warnings fixed!" -ForegroundColor Green
}

# Phase 3: Code formatting
Write-Host "`n🎨 Phase 3: Code formatting..." -ForegroundColor Yellow
Write-Host "  📐 Running cargo fmt..."
cargo fmt

# Phase 4: Validation
Write-Host "`n✅ Phase 4: Validation..." -ForegroundColor Yellow
Write-Host "  🧪 Running tests..."
$test_result = cargo test --quiet
if ($LASTEXITCODE -eq 0) {
    Write-Host "  ✅ All tests pass!" -ForegroundColor Green
} else {
    Write-Host "  ❌ Some tests failed. Check output above." -ForegroundColor Red
}

Write-Host "  🔍 Final check..."
$final_check = cargo check --all-targets --all-features --quiet
if ($LASTEXITCODE -eq 0) {
    Write-Host "  ✅ Code compiles successfully!" -ForegroundColor Green
} else {
    Write-Host "  ❌ Compilation errors found. Check output above." -ForegroundColor Red
}

Write-Host "`n🎉 Cleanup script completed!" -ForegroundColor Green
Write-Host "📋 Check CLEANUP_TRACKING.md for manual fixes still needed." -ForegroundColor Cyan

# Summary
Write-Host "`n📊 Summary:" -ForegroundColor Cyan
Write-Host "  🔧 Auto-fixes applied: Multiple categories" -ForegroundColor White
Write-Host "  ⚠️  Remaining warnings: $warning_count" -ForegroundColor White
Write-Host "  📝 Next steps: Manual fixes in CLEANUP_TRACKING.md" -ForegroundColor White
