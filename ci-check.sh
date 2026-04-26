#!/bin/bash
################################################################################
#
#    Copyright (c) 2026.
#    Haixing Hu, Qubit Co. Ltd.
#
#    All rights reserved.
#
################################################################################
#
# Local CI check script
# Run this script before committing code to ensure it passes all CircleCI checks
#

set -euo pipefail

# Color definitions
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Print colored messages
print_step() {
    echo -e "${BLUE}==>${NC} $1"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

require_command() {
    if ! command -v "$1" > /dev/null 2>&1; then
        print_error "Required command '$1' was not found"
        exit 1
    fi
}

run_security_audit() {
    if cargo audit; then
        print_success "Security audit passed, no known vulnerabilities found"
        return
    fi

    print_warning "cargo audit failed. Retrying with the cached advisory database to distinguish network fetch failures from real advisories."
    if cargo audit --no-fetch --stale; then
        print_success "Security audit passed using the cached advisory database"
        print_warning "The online advisory database fetch failed; CI should still verify against the latest RustSec database."
        return
    fi

    print_error "Security audit found issues"
    echo ""
    echo "Please review the security issues and consider:"
    echo "  1. Update dependencies: cargo update"
    echo "  2. View details: cargo audit"
    echo "  3. If unable to fix immediately, temporarily ignore in .cargo-audit.toml"
    exit 1
}

require_command cargo
require_command rustup

# Switch to script directory
SCRIPT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
cd "$SCRIPT_DIR"

echo "🚀 Starting local CI checks..."
echo ""

# Check 1: Code formatting
print_step "1/7 Checking code format (cargo +nightly fmt)..."

# Check if nightly toolchain is installed
if ! rustup toolchain list | grep -q nightly; then
    print_warning "Nightly toolchain not found, installing..."
    rustup toolchain install nightly
fi

if cargo +nightly fmt -- --check > /dev/null 2>&1; then
    print_success "Code format check passed"
else
    print_error "Code format check failed"
    echo ""
    echo "Please run the following command to fix formatting issues:"
    echo "  cargo +nightly fmt"
    echo "Or use the format script:"
    echo "  ./format.sh"
    exit 1
fi
echo ""

# Check 2: Clippy linting
print_step "2/7 Running Clippy checks (cargo +nightly clippy)..."
if cargo +nightly clippy --all-targets --all-features -- -D warnings; then
    print_success "Clippy checks passed"
else
    print_error "Clippy found issues"
    echo ""
    echo "Please try to auto-fix with:"
    echo "  cargo +nightly clippy --fix --all-targets --all-features"
    exit 1
fi
echo ""

# Check 3: Build project
print_step "3/7 Building project (cargo build)..."
if cargo build --verbose > /dev/null 2>&1; then
    print_success "Debug build succeeded"
else
    print_error "Debug build failed"
    cargo build --verbose
    exit 1
fi

if cargo build --release --verbose > /dev/null 2>&1; then
    print_success "Release build succeeded"
else
    print_error "Release build failed"
    cargo build --release --verbose
    exit 1
fi
echo ""

# Check 4: Run tests
print_step "4/7 Running tests (cargo test)..."
if cargo test --verbose; then
    print_success "All tests passed"
else
    print_error "Tests failed"
    exit 1
fi
echo ""

# Check 5: Build documentation
print_step "5/7 Building documentation (rustdoc -D warnings)..."
if RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --verbose; then
    print_success "Documentation build passed"
else
    print_error "Documentation build failed"
    exit 1
fi
echo ""

# Check 6: Code coverage
print_step "6/7 Generating and checking JSON coverage report..."
if command -v cargo-llvm-cov > /dev/null 2>&1; then
    ./coverage.sh json
    print_success "Coverage report passed thresholds"
else
    print_error "cargo-llvm-cov is not installed"
    echo "Installation instructions:"
    echo "  cargo install cargo-llvm-cov"
    echo "  rustup component add llvm-tools-preview   # on the same toolchain as this project (see: rustup show active-toolchain)"
    exit 1
fi
echo ""

# Check 7: Security audit
print_step "7/7 Running security audit (cargo audit)..."
if command -v cargo-audit > /dev/null 2>&1; then
    run_security_audit
else
    print_error "cargo-audit is not installed"
    echo "Installation instructions:"
    echo "  cargo install cargo-audit"
    exit 1
fi
echo ""

# All checks passed
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
print_success "All checks passed! 🎉"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "Your code is ready to commit."
echo "After pushing, CircleCI will automatically run the same checks."
echo ""
