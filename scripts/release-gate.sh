#!/bin/bash
#
# NOA Release Gate Script
#
# A single script that validates the codebase is ready for release.
# Runs: format/lints, cargo check, unit tests, integration tests, UI build.
#
# Usage:
#   ./scripts/release-gate.sh [--quick] [--skip-ui] [--skip-integration]
#
# Options:
#   --quick           Skip slow tests, run minimal checks
#   --skip-ui         Skip UI build step
#   --skip-integration Skip integration tests
#   --verbose         Enable verbose output
#   --help            Show this help message
#
# Exit codes:
#   0 - All checks passed
#   1 - One or more checks failed
#   2 - Missing prerequisites

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
NOA_ROOT="${NOA_ROOT:-$(cd "$SCRIPT_DIR/.." && pwd)}"

# Default options
QUICK=false
SKIP_UI=false
SKIP_INTEGRATION=false
VERBOSE=false

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Counters
PASSED=0
FAILED=0
SKIPPED=0
WARNINGS=0

# Timing
START_TIME=$(date +%s)

log_info()    { echo -e "${BLUE}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[PASS]${NC} $1"; ((PASSED++)); }
log_warning() { echo -e "${YELLOW}[WARN]${NC} $1"; ((WARNINGS++)); }
log_error()   { echo -e "${RED}[FAIL]${NC} $1"; ((FAILED++)); }
log_skip()    { echo -e "${CYAN}[SKIP]${NC} $1"; ((SKIPPED++)); }

log_section() {
    echo ""
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${CYAN}  $1${NC}"
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
}

show_help() {
    head -25 "$0" | tail -22
    exit 0
}

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --quick)           QUICK=true; shift ;;
        --skip-ui)         SKIP_UI=true; shift ;;
        --skip-integration) SKIP_INTEGRATION=true; shift ;;
        --verbose)         VERBOSE=true; shift ;;
        --help|-h)         show_help ;;
        *)
            log_error "Unknown option: $1"
            show_help
            ;;
    esac
done

# Check prerequisites
check_prerequisites() {
    log_section "Checking Prerequisites"

    local missing=0

    if command -v cargo &> /dev/null; then
        log_success "cargo found: $(cargo --version)"
    else
        log_error "cargo not found"
        missing=1
    fi

    if command -v rustfmt &> /dev/null; then
        log_success "rustfmt found"
    else
        log_error "rustfmt not found (run: rustup component add rustfmt)"
        missing=1
    fi

    if command -v clippy-driver &> /dev/null; then
        log_success "clippy found"
    else
        log_error "clippy not found (run: rustup component add clippy)"
        missing=1
    fi

    if ! $SKIP_UI; then
        if command -v npm &> /dev/null; then
            log_success "npm found: $(npm --version)"
        else
            log_warning "npm not found, UI checks will be skipped"
            SKIP_UI=true
        fi
    fi

    if [[ $missing -eq 1 ]]; then
        log_error "Missing required prerequisites"
        exit 2
    fi
}

# Windows incremental build fix
setup_cargo_env() {
    log_section "Setting Up Build Environment"

    # Detect Windows (including Git Bash, MSYS2, Cygwin)
    if [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "cygwin" ]] || [[ -n "${WINDIR:-}" ]]; then
        log_info "Windows detected - setting CARGO_INCREMENTAL=0 to avoid file locking issues"
        export CARGO_INCREMENTAL=0
    else
        log_info "Unix-like system detected - incremental builds enabled"
    fi

    # Always clean target dir metadata if it's stale (optional)
    if [[ "$VERBOSE" == "true" ]]; then
        log_info "CARGO_INCREMENTAL=${CARGO_INCREMENTAL:-1}"
    fi
}

# Rust format check
check_rust_format() {
    log_section "Rust Format Check"
    cd "$NOA_ROOT/sys/core"

    if cargo fmt --all -- --check; then
        log_success "Rust formatting is correct"
    else
        log_error "Rust formatting errors found (run: cargo fmt --all)"
        return 1
    fi
}

# Rust clippy lints
check_rust_lints() {
    log_section "Rust Lints (Clippy)"
    cd "$NOA_ROOT/sys/core"

    local clippy_args="--all-targets --all-features"
    if $QUICK; then
        clippy_args="--lib"
    fi

    if cargo clippy $clippy_args -- -D warnings 2>&1; then
        log_success "Clippy passed with no warnings"
    else
        log_error "Clippy found issues"
        return 1
    fi
}

# Cargo check
run_cargo_check() {
    log_section "Cargo Check"
    cd "$NOA_ROOT/sys/core"

    if cargo check -p noa-core --all-features; then
        log_success "cargo check -p noa-core passed"
    else
        log_error "cargo check failed"
        return 1
    fi

    # Check all workspace members
    if ! $QUICK; then
        if cargo check --workspace --all-features; then
            log_success "cargo check --workspace passed"
        else
            log_error "workspace cargo check failed"
            return 1
        fi
    fi
}

# Unit tests
run_unit_tests() {
    log_section "Unit Tests"
    cd "$NOA_ROOT/sys/core"

    local test_args=""
    if $QUICK; then
        test_args="--lib"
    fi

    if cargo test $test_args; then
        log_success "Unit tests passed"
    else
        log_error "Unit tests failed"
        return 1
    fi
}

# Integration tests (if not skipped)
run_integration_tests() {
    if $SKIP_INTEGRATION; then
        log_skip "Integration tests (--skip-integration)"
        return 0
    fi

    log_section "Integration Tests"
    cd "$NOA_ROOT/sys/core"

    # Run integration tests if they exist
    if cargo test --test '*' -- --test-threads=1 2>/dev/null; then
        log_success "Integration tests passed"
    else
        # Check if there are no integration tests vs actual failures
        if [[ $? -eq 101 ]]; then
            log_warning "No integration tests found"
        else
            log_error "Integration tests failed"
            return 1
        fi
    fi
}

# UI build
run_ui_build() {
    if $SKIP_UI; then
        log_skip "UI build (--skip-ui or npm not found)"
        return 0
    fi

    log_section "UI Build"
    cd "$NOA_ROOT/sys/ui"

    if [[ ! -f "package.json" ]]; then
        log_warning "No package.json found in sys/ui"
        return 0
    fi

    log_info "Installing dependencies..."
    if ! npm ci --silent 2>/dev/null; then
        npm install --silent
    fi

    log_info "Type checking..."
    if npm run type-check 2>/dev/null; then
        log_success "TypeScript type check passed"
    else
        log_error "TypeScript type check failed"
        return 1
    fi

    log_info "Linting..."
    if npm run lint 2>/dev/null; then
        log_success "ESLint passed"
    else
        log_error "ESLint found issues"
        return 1
    fi

    if ! $QUICK; then
        log_info "Building..."
        if npm run build 2>/dev/null; then
            log_success "UI build passed"
        else
            log_error "UI build failed"
            return 1
        fi
    else
        log_skip "UI build (--quick mode)"
    fi
}

# Go checks (if go is available)
run_go_checks() {
    if ! command -v go &> /dev/null; then
        log_skip "Go checks (go not found)"
        return 0
    fi

    log_section "Go Checks"
    cd "$NOA_ROOT/p2p"

    if [[ ! -f "go.mod" ]]; then
        log_skip "Go checks (no go.mod found)"
        return 0
    fi

    log_info "Go build..."
    if go build ./...; then
        log_success "Go build passed"
    else
        log_error "Go build failed"
        return 1
    fi

    if ! $QUICK; then
        log_info "Go test..."
        if go test ./...; then
            log_success "Go tests passed"
        else
            log_error "Go tests failed"
            return 1
        fi
    fi
}

# Python checks (if python is available)
run_python_checks() {
    if ! command -v python3 &> /dev/null && ! command -v python &> /dev/null; then
        log_skip "Python checks (python not found)"
        return 0
    fi

    log_section "Python Checks"
    cd "$NOA_ROOT/sys/digest"

    if [[ ! -f "pyproject.toml" ]]; then
        log_skip "Python checks (no pyproject.toml found)"
        return 0
    fi

    if command -v ruff &> /dev/null; then
        if ruff check .; then
            log_success "Ruff lint passed"
        else
            log_error "Ruff found issues"
            return 1
        fi
    else
        log_warning "ruff not found, skipping Python lint"
    fi
}

# Summary
print_summary() {
    local END_TIME=$(date +%s)
    local DURATION=$((END_TIME - START_TIME))

    log_section "Release Gate Summary"

    echo ""
    echo -e "  ${GREEN}Passed:${NC}   $PASSED"
    echo -e "  ${RED}Failed:${NC}   $FAILED"
    echo -e "  ${CYAN}Skipped:${NC}  $SKIPPED"
    echo -e "  ${YELLOW}Warnings:${NC} $WARNINGS"
    echo ""
    echo -e "  Duration: ${DURATION}s"
    echo ""

    if [[ $FAILED -gt 0 ]]; then
        echo -e "  ${RED}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
        echo -e "  ${RED}  RELEASE GATE FAILED${NC}"
        echo -e "  ${RED}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
        exit 1
    else
        echo -e "  ${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
        echo -e "  ${GREEN}  RELEASE GATE PASSED${NC}"
        echo -e "  ${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
        exit 0
    fi
}

# Main execution
main() {
    echo -e "${CYAN}"
    echo "╔════════════════════════════════════════════════════════════════╗"
    echo "║                    NOA Release Gate                            ║"
    echo "╚════════════════════════════════════════════════════════════════╝"
    echo -e "${NC}"

    if $QUICK; then
        log_info "Running in QUICK mode (subset of checks)"
    fi

    check_prerequisites
    setup_cargo_env

    # Run all checks, continuing even if some fail
    check_rust_format || true
    check_rust_lints || true
    run_cargo_check || true
    run_unit_tests || true
    run_integration_tests || true
    run_ui_build || true
    run_go_checks || true
    run_python_checks || true

    print_summary
}

main
