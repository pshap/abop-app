#!/usr/bin/env bash
# Run every check a commit must pass: formatting, clippy and the test suite.
# Used by the pre-commit hook (.githooks/pre-commit); safe to run by hand.
set -uo pipefail
cd "$(git rev-parse --show-toplevel)"

fail() {
    echo
    echo "✗ $1"
    echo "  Fix the problem above and commit again. Do not bypass with --no-verify."
    exit 1
}

echo "→ cargo fmt --check"
cargo fmt --all -- --check >/dev/null 2>&1 || {
    cargo fmt --all -- --check 2>&1 | head -40
    fail "Formatting is off. Run: cargo fmt --all"
}

echo "→ cargo clippy"
cargo clippy --workspace --all-targets --quiet -- -D warnings || fail "Clippy found problems."

echo "→ cargo nextest"
if command -v cargo-nextest >/dev/null 2>&1; then
    cargo nextest run --workspace --status-level fail --final-status-level fail \
        || fail "Tests failed."
else
    cargo test --workspace --quiet || fail "Tests failed."
fi

echo "✓ All checks passed"
