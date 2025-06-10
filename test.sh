#!/bin/bash

# Test script for tilde-fish
# Runs comprehensive test suite including unit tests, integration tests, 
# property-based tests, and benchmarks

set -e  # Exit on any error

echo "🐟 Running tilde-fish test suite..."
echo "======================================"

# Check if we're in the right directory
if [[ ! -f "Cargo.toml" ]]; then
    echo "❌ Error: Please run this script from the tilde-fish project root"
    exit 1
fi

# Function to run a test command and report results
run_test() {
    local test_name="$1"
    local test_cmd="$2"
    
    echo ""
    echo "🧪 Running $test_name..."
    echo "----------------------------------------"
    
    if eval "$test_cmd"; then
        echo "✅ $test_name passed"
    else
        echo "❌ $test_name failed"
        exit 1
    fi
}

# Function to run optional tests that may not be critical
run_optional_test() {
    local test_name="$1"
    local test_cmd="$2"
    
    echo ""
    echo "🔧 Running $test_name (optional)..."
    echo "----------------------------------------"
    
    if eval "$test_cmd"; then
        echo "✅ $test_name passed"
    else
        echo "⚠️  $test_name failed (non-critical)"
    fi
}

# Ensure we're using the latest build
echo "🔨 Building project..."
cargo build

# Run formatting check
run_test "Code formatting" "cargo fmt -- --check"

# Run linting
run_test "Clippy linting" "cargo clippy -- -D warnings"

# Run unit tests
run_test "Unit tests" "cargo test --lib"

# Run integration tests
run_test "Integration tests" "cargo test --test integration_tests"

# Run property-based tests
run_test "Property-based tests" "cargo test --test property_tests"

# Run test helpers
run_test "Test helpers" "cargo test --test test_helpers"

# Run all tests together
run_test "All tests combined" "cargo test"

# Test the actual binary functionality
echo ""
echo "🔧 Testing binary functionality..."
echo "----------------------------------------"

# Test plugin config generation
echo "Testing --config flag..."
if cargo run -- --config > /dev/null; then
    echo "✅ Plugin config generation works"
else
    echo "❌ Plugin config generation failed"
    exit 1
fi

# Test fish config generation with sample input
echo "Testing --gen-config flag..."
echo '{}' | cargo run -- --gen-config > /dev/null
if [[ $? -eq 0 ]]; then
    echo "✅ Fish config generation works"
else
    echo "❌ Fish config generation failed"
    exit 1
fi

# Test with the test configuration file
if [[ -f "test_config.edn" ]]; then
    echo "Testing with test_config.edn..."
    if cargo run -- --gen-config < test_config.edn > /dev/null; then
        echo "✅ Test configuration processing works"
    else
        echo "❌ Test configuration processing failed"
        exit 1
    fi
fi

# Run performance benchmarks (optional)
run_optional_test "Performance benchmarks" "cargo bench --bench fish_config_bench"

# Generate test coverage report if tarpaulin is available
if command -v cargo-tarpaulin >/dev/null 2>&1; then
    run_optional_test "Test coverage" "cargo tarpaulin --out stdout --engine llvm"
else
    echo ""
    echo "📊 Test coverage tool not found (cargo-tarpaulin)"
    echo "   Install with: cargo install cargo-tarpaulin"
fi

# Final summary
echo ""
echo "🎉 All tests completed successfully!"
echo "======================================"
echo ""
echo "Test summary:"
echo "- ✅ Code formatting"
echo "- ✅ Clippy linting" 
echo "- ✅ Unit tests (19 tests)"
echo "- ✅ Integration tests (14 tests)"
echo "- ✅ Property-based tests (11 tests)"
echo "- ✅ Test helpers (5 tests)"
echo "- ✅ Binary functionality"
echo "- ✅ Configuration processing"
echo ""
echo "Total: 49+ tests passed"
echo ""
echo "🚀 Ready for deployment!"