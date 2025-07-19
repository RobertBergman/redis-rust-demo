# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Redis Rust Demo Application designed to demonstrate various Redis features, patterns, and best practices using Rust.

### Project Specification

The complete project specification is documented in `redis-rust-demo-spec.md`. This specification includes:

- **Project Goals**: Core objectives and demonstration targets
- **Technical Stack**: Rust, Redis, Tokio, and other dependencies
- **Application Architecture**: Module structure and organization
- **Feature Demonstrations**: 
  - Basic Redis operations (strings, keys, expiration)
  - Data structures (lists, sets, sorted sets, hashes, bitmaps, HyperLogLog, geospatial)
  - Advanced features (Pub/Sub, Streams, transactions, Lua scripting, pipelining)
  - Design patterns (caching, rate limiting, distributed locking, session management, leaderboards)
- **CLI Interface**: Command structure and usage examples
- **Implementation Details**: Error handling, connection management, async patterns
- **Testing Strategy**: Unit tests, integration tests, benchmarks
- **Performance Considerations**: Optimization strategies and metrics

**IMPORTANT**: Always refer to `redis-rust-demo-spec.md` when implementing new features or understanding the project structure. The specification is the authoritative source for all architectural decisions and feature requirements.

## Rust Development Guidelines

Please refer to `RUST.md` for comprehensive guidance on:
- Common Rust errors and how to avoid them
- Ownership and borrowing best practices
- Lifetime management
- Async/await patterns
- Error handling conventions
- Performance optimization tips
- Testing best practices
- Redis-specific Rust patterns

Always follow the patterns and practices outlined in RUST.md when writing or modifying Rust code in this project.

## Development Commands

Once the project is initialized, use these commands:

```bash
# Initialize the Rust project (if not already done)
cargo init --name redis-rust-demo

# Build the project
cargo build
cargo build --release

# Run tests
cargo test
cargo test -- --nocapture  # Show println! output
cargo test <test_name>      # Run specific test

# Run the application
cargo run -- <subcommand>
cargo run -- basic strings
cargo run -- basic lists
cargo run -- basic sets
cargo run -- basic hashes
cargo run -- rust-errors      # Demonstrate common Rust errors and fixes
cargo run -- pattern cache --size 1000

# Format code
cargo fmt

# Lint code
cargo clippy -- -D warnings

# Check types without building
cargo check

# Run benchmarks
cargo bench

# Generate documentation
cargo doc --open

# Docker commands for Redis
docker-compose up -d        # Start Redis container
docker-compose down         # Stop Redis container
```

## Architecture Overview

The application follows a modular architecture as specified in `redis-rust-demo-spec.md`:

### Core Modules

1. **CLI Module** (`src/cli/`)
   - Handles command-line interface using clap
   - Routes commands to appropriate demo modules

2. **Demo Modules** (`src/demos/`)
   - Each file implements specific Redis feature demonstrations
   - Organized by functionality (basic operations, data structures, advanced features)
   - Pattern implementations in `demos/patterns/` subdirectory

3. **Models** (`src/models/`)
   - Data structures used across demonstrations
   - Serializable types for Redis storage

4. **Utils** (`src/utils/`)
   - `redis_client.rs`: Connection management and pooling
   - `error.rs`: Custom error types using thiserror

### Key Design Decisions

1. **Async-First**: All Redis operations use Tokio and async/await
2. **Connection Pooling**: Uses r2d2 for efficient connection management
3. **Error Handling**: Comprehensive error types with thiserror
4. **Modular Demos**: Each Redis feature is a separate, runnable demonstration

## Redis Feature Implementation Guide

When implementing Redis features, follow this pattern:

1. Create a new module in `src/demos/` for the feature
2. Implement both synchronous and asynchronous versions where applicable
3. Include error handling and logging
4. Add CLI command in `src/cli/commands.rs`
5. Write integration tests in `tests/`
6. Update documentation with usage examples

## Testing Approach

- Unit tests: Mock Redis connections for isolated testing
- Integration tests: Use Docker Redis instance for real operations
- Benchmark tests: Measure performance of different approaches
- Use `#[tokio::test]` for async test functions

## Dependencies to Add

When initializing the project, add these to `Cargo.toml`:

```toml
[dependencies]
redis = { version = "0.24", features = ["tokio-comp", "connection-manager"] }
tokio = { version = "1", features = ["full"] }
clap = { version = "4", features = ["derive"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tracing = "0.1"
tracing-subscriber = "0.3"
thiserror = "1"
anyhow = "1"

[dev-dependencies]
criterion = "0.5"
```

## Project Status

**Current State**: Implementation phase - Core features implemented with comprehensive test coverage

### Implemented Features
- ✅ Basic Redis operations (strings, keys management)
- ✅ Data structures (lists, sets, hashes)
- ✅ Connection pooling and management
- ✅ CLI interface with subcommands
- ✅ Error handling framework
- ✅ Comprehensive test suite (90%+ coverage)
- ✅ Rust errors demonstration (educational tool for common Rust pitfalls)

### Pending Features (per specification)
- ⏳ Sorted sets, bitmaps, HyperLogLog, geospatial data structures
- ⏳ Pub/Sub demonstrations
- ⏳ Redis Streams
- ⏳ Transactions and Lua scripting
- ⏳ Advanced patterns (rate limiting, distributed locking)
- ⏳ Performance benchmarks
- ⏳ Docker containerization

When implementing new features, always refer to `redis-rust-demo-spec.md` for the complete requirements and expected behavior.