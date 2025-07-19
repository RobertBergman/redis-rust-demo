#!/bin/bash

# Exit on error
set -e

echo "🧪 Running tests with coverage..."

# Install tarpaulin if not installed
if ! command -v cargo-tarpaulin &> /dev/null; then
    echo "📦 Installing cargo-tarpaulin..."
    cargo install cargo-tarpaulin
fi

# Make sure Redis is running
if ! nc -z localhost 6379; then
    echo "❌ Redis is not running on localhost:6379"
    echo "Please start Redis with: docker-compose up -d"
    exit 1
fi

# Clean up test databases
echo "🧹 Cleaning up test databases..."
redis-cli -n 14 FLUSHDB > /dev/null 2>&1 || true
redis-cli -n 15 FLUSHDB > /dev/null 2>&1 || true

# Run tests with coverage
echo "🚀 Running tests with coverage..."
cargo tarpaulin \
    --out Html \
    --out Lcov \
    --output-dir target/coverage \
    --workspace \
    --exclude-files "*/tests/*" \
    --exclude-files "*/benches/*" \
    --exclude-files "*/examples/*" \
    --ignore-panics \
    --ignore-tests \
    --branch \
    --skip-clean \
    --timeout 300 \
    --features "test-utilities"

# Display coverage summary
echo ""
echo "📊 Coverage Report:"
cargo tarpaulin --print-summary

echo ""
echo "✅ Coverage report generated at: target/coverage/tarpaulin-report.html"
echo "📈 LCOV report generated at: target/coverage/lcov.info"