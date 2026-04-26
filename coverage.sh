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
# Code coverage testing script
# Uses cargo-llvm-cov to generate code coverage reports
#

set -euo pipefail

MIN_FUNCTION_COVERAGE="${MIN_FUNCTION_COVERAGE:-100}"
MIN_LINE_COVERAGE="${MIN_LINE_COVERAGE:-98}"
MIN_REGION_COVERAGE="${MIN_REGION_COVERAGE:-98}"

print_usage() {
    echo "Usage: ./coverage.sh [format] [options]"
    echo ""
    echo "Format options:"
    echo "  html       Generate HTML report and open in browser (default)"
    echo "  text       Output text format report to terminal"
    echo "  lcov       Generate LCOV format report"
    echo "  json       Generate JSON format report and enforce coverage thresholds"
    echo "  cobertura  Generate Cobertura XML format report"
    echo "  all        Generate all format reports and enforce coverage thresholds"
    echo "  help       Show this help information"
    echo ""
    echo "Options:"
    echo "  --clean    Clean old coverage data and build cache before running"
    echo "             By default, cached builds are used to speed up compilation"
    echo ""
    echo "Dependencies:"
    echo "  cargo, cargo-llvm-cov"
    echo "  jq         Required for JSON/all threshold checks"
    echo ""
    echo "Coverage thresholds for JSON/all:"
    echo "  functions: ${MIN_FUNCTION_COVERAGE}%"
    echo "  lines:     > ${MIN_LINE_COVERAGE}%"
    echo "  regions:   > ${MIN_REGION_COVERAGE}%"
    echo ""
    echo "Performance tips:"
    echo "  • First run will be slower (needs to compile all dependencies)"
    echo "  • Subsequent runs will be much faster (using cache)"
    echo "  • Only use --clean when dependencies are updated or major code changes"
    echo ""
    echo "Examples:"
    echo "  ./coverage.sh              # Generate HTML report (using cache)"
    echo "  ./coverage.sh text         # Output text report (using cache)"
    echo "  ./coverage.sh json         # Generate JSON and enforce thresholds"
    echo "  ./coverage.sh --clean      # Clean then generate HTML report"
    echo "  ./coverage.sh html --clean # Clean then generate HTML report"
    echo "  ./coverage.sh all --clean  # Clean then generate all formats"
}

require_command() {
    if ! command -v "$1" > /dev/null 2>&1; then
        echo "❌ Error: required command '$1' was not found"
        exit 1
    fi
}

check_json_coverage() {
    local coverage_json="$1"

    require_command jq

    if [ ! -f "$coverage_json" ]; then
        echo "❌ Error: coverage JSON not found: $coverage_json"
        exit 1
    fi

    local failures
    failures=$(jq -r \
        --argjson min_functions "$MIN_FUNCTION_COVERAGE" \
        --argjson min_lines "$MIN_LINE_COVERAGE" \
        --argjson min_regions "$MIN_REGION_COVERAGE" \
        '
        .data[0].files[]
        | select(
            (.summary.functions.percent < $min_functions)
            or (.summary.lines.percent <= $min_lines)
            or (.summary.regions.percent <= $min_regions)
        )
        | "\(.filename): functions=\(.summary.functions.percent)% lines=\(.summary.lines.percent)% regions=\(.summary.regions.percent)%"
        ' "$coverage_json")

    if [ -n "$failures" ]; then
        echo "❌ Coverage thresholds failed:"
        echo "$failures"
        echo ""
        echo "Required: functions = ${MIN_FUNCTION_COVERAGE}%, lines > ${MIN_LINE_COVERAGE}%, regions > ${MIN_REGION_COVERAGE}%"
        exit 1
    fi

    echo "✅ Coverage thresholds satisfied"
    echo "   Required: functions = ${MIN_FUNCTION_COVERAGE}%, lines > ${MIN_LINE_COVERAGE}%, regions > ${MIN_REGION_COVERAGE}%"
}

require_command cargo
require_command cargo-llvm-cov

echo "🔍 Starting code coverage testing..."

# Switch to project directory
SCRIPT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
cd "$SCRIPT_DIR"

# Detect package name from Cargo.toml
if [ -f "Cargo.toml" ]; then
    PACKAGE_NAME=$(grep "^name = " Cargo.toml | head -n 1 | sed 's/name = "\(.*\)"/\1/')
    if [ -z "$PACKAGE_NAME" ]; then
        echo "❌ Error: unable to detect package name from Cargo.toml"
        exit 1
    fi
    echo "📦 Detected package: $PACKAGE_NAME"
else
    echo "❌ Error: Cargo.toml not found in current directory"
    exit 1
fi

# Get current directory absolute path to filter coverage
CURRENT_CRATE_DIR=$(pwd)
echo "📁 Coverage will only include files in: $CURRENT_CRATE_DIR"

# Build regex pattern to exclude third-party code and other workspace members
CURRENT_CRATE_NAME=$(basename "$CURRENT_CRATE_DIR")
WORKSPACE_ROOT=$(cd "$(dirname "$0")/.." && pwd)

# Create list of other workspace crates to exclude (sibling directories under workspace root)
OTHER_CRATES=""
for crate_dir in "$WORKSPACE_ROOT"/*/; do
    [ -d "$crate_dir" ] || continue
    crate_name=$(basename "$crate_dir")
    if [ "$crate_name" != "$CURRENT_CRATE_NAME" ]; then
        if [ -z "$OTHER_CRATES" ]; then
            OTHER_CRATES="$crate_name"
        else
            OTHER_CRATES="$OTHER_CRATES|$crate_name"
        fi
    fi
done

# Exclude: cargo registry, rustup, and other workspace crates
# Using simple alternation for clarity
EXCLUDE_PATTERN="(\.cargo/registry|\.rustup/"
if [ -n "$OTHER_CRATES" ]; then
    EXCLUDE_PATTERN="$EXCLUDE_PATTERN|/($OTHER_CRATES)/"
fi
EXCLUDE_PATTERN="$EXCLUDE_PATTERN)"
echo "🚫 Excluding: .cargo/registry, .rustup, and other workspace members"

# Parse arguments, check if cleanup is needed
CLEAN_FLAG=""
FORMAT_ARG=""

for arg in "$@"; do
    case "$arg" in
        --clean)
            CLEAN_FLAG="yes"
            ;;
        help|--help|-h)
            print_usage
            exit 0
            ;;
        *)
            if [ -n "$FORMAT_ARG" ]; then
                echo "❌ Error: multiple formats specified ('$FORMAT_ARG' and '$arg')"
                print_usage
                exit 1
            fi
            FORMAT_ARG="$arg"
            ;;
    esac
done

# Default format is html
FORMAT_ARG="${FORMAT_ARG:-html}"

if [ "$FORMAT_ARG" = "json" ] || [ "$FORMAT_ARG" = "all" ]; then
    require_command jq
fi

# If --clean option is specified, clean old data
if [ "$CLEAN_FLAG" = "yes" ]; then
    echo "🧹 Cleaning old coverage data..."
    cargo llvm-cov clean
else
    echo "ℹ️  Using cached build (use --clean option if you need to clean cache)"
fi

# cargo-llvm-cov does not create parent directories for --json/--lcov/--cobertura outputs
mkdir -p target/llvm-cov

# Run tests and generate coverage reports
case "$FORMAT_ARG" in
    html)
        echo "📊 Generating HTML format coverage report..."
        cargo llvm-cov --package "$PACKAGE_NAME" --html --open \
            --ignore-filename-regex "$EXCLUDE_PATTERN"
        echo "✅ HTML report generated and opened in browser"
        echo "   Report location: target/llvm-cov/html/index.html"
        ;;

    text)
        echo "📊 Generating text format coverage report..."
        cargo llvm-cov --package "$PACKAGE_NAME" \
            --ignore-filename-regex "$EXCLUDE_PATTERN"
        ;;

    lcov)
        echo "📊 Generating LCOV format coverage report..."
        cargo llvm-cov --package "$PACKAGE_NAME" --lcov --output-path target/llvm-cov/lcov.info \
            --ignore-filename-regex "$EXCLUDE_PATTERN"
        echo "✅ LCOV report generated"
        echo "   Report location: target/llvm-cov/lcov.info"
        ;;

    json)
        echo "📊 Generating JSON format coverage report..."
        cargo llvm-cov --package "$PACKAGE_NAME" --json --output-path target/llvm-cov/coverage.json \
            --ignore-filename-regex "$EXCLUDE_PATTERN"
        echo "✅ JSON report generated"
        echo "   Report location: target/llvm-cov/coverage.json"
        check_json_coverage target/llvm-cov/coverage.json
        ;;

    cobertura)
        echo "📊 Generating Cobertura XML format coverage report..."
        cargo llvm-cov --package "$PACKAGE_NAME" --cobertura --output-path target/llvm-cov/cobertura.xml \
            --ignore-filename-regex "$EXCLUDE_PATTERN"
        echo "✅ Cobertura report generated"
        echo "   Report location: target/llvm-cov/cobertura.xml"
        ;;

    all)
        echo "📊 Generating all format coverage reports..."

        echo "  - Running tests and collecting coverage data..."
        cargo llvm-cov --package "$PACKAGE_NAME" --no-report \
            --ignore-filename-regex "$EXCLUDE_PATTERN"

        # HTML
        echo "  - Generating HTML report..."
        cargo llvm-cov report --package "$PACKAGE_NAME" --html --output-dir target/llvm-cov \
            --ignore-filename-regex "$EXCLUDE_PATTERN"

        # LCOV
        echo "  - Generating LCOV report..."
        cargo llvm-cov report --package "$PACKAGE_NAME" --lcov --output-path target/llvm-cov/lcov.info \
            --ignore-filename-regex "$EXCLUDE_PATTERN"

        # JSON
        echo "  - Generating JSON report..."
        cargo llvm-cov report --package "$PACKAGE_NAME" --json --output-path target/llvm-cov/coverage.json \
            --ignore-filename-regex "$EXCLUDE_PATTERN"
        check_json_coverage target/llvm-cov/coverage.json

        # Cobertura
        echo "  - Generating Cobertura XML report..."
        cargo llvm-cov report --package "$PACKAGE_NAME" --cobertura --output-path target/llvm-cov/cobertura.xml \
            --ignore-filename-regex "$EXCLUDE_PATTERN"

        # Text
        echo "  - Generating text report..."
        cargo llvm-cov report --package "$PACKAGE_NAME" --text --output-path target/llvm-cov/coverage.txt \
            --ignore-filename-regex "$EXCLUDE_PATTERN"

        echo "✅ All format reports generated"
        echo "   HTML:      target/llvm-cov/html/index.html"
        echo "   LCOV:      target/llvm-cov/lcov.info"
        echo "   JSON:      target/llvm-cov/coverage.json"
        echo "   Cobertura: target/llvm-cov/cobertura.xml"
        echo "   Text:      target/llvm-cov/coverage.txt"
        ;;

    *)
        echo "❌ Error: Unknown format '$1'"
        print_usage
        exit 1
        ;;
esac

echo "✅ Code coverage testing completed!"
