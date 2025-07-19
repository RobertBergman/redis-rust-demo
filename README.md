# Redis Rust Demo Application

A comprehensive Rust application demonstrating Redis features, patterns, and best practices. This project serves as both a learning resource and a reference implementation for Redis integration in Rust applications.

## Features

### Implemented
- ✅ **Basic Redis Operations**: String operations, key management, expiration
- ✅ **Data Structures**: Lists, Sets, Hashes with practical examples
- ✅ **Connection Management**: Async connection pooling with r2d2
- ✅ **Error Handling**: Comprehensive error types with thiserror
- ✅ **CLI Interface**: Intuitive command-line interface with subcommands
- ✅ **Educational Tools**: Rust error demonstrations showing common pitfalls
- ✅ **Test Coverage**: 90%+ test coverage with unit and integration tests

### Coming Soon
- ⏳ Sorted Sets, Bitmaps, HyperLogLog, Geospatial data
- ⏳ Pub/Sub demonstrations
- ⏳ Redis Streams
- ⏳ Transactions and Lua scripting
- ⏳ Advanced patterns (rate limiting, distributed locking)

## Quick Start

### Prerequisites
- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))
- Redis 7.0+ (running on localhost:6379)
- Docker (optional, for Redis container)

### Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/redis-rust-demo.git
cd redis-rust-demo
```

2. Start Redis using Docker:
```bash
docker-compose up -d
```

3. Build the project:
```bash
cargo build --release
```

### Usage

Run various Redis demonstrations:

```bash
# Test Redis connection
cargo run -- ping

# Basic operations
cargo run -- basic strings   # String operations and key management
cargo run -- basic lists     # List operations and message queue patterns
cargo run -- basic sets      # Set operations and unique visitor tracking
cargo run -- basic hashes    # Hash operations and shopping cart example

# Educational tools
cargo run -- rust-errors     # Common Rust errors and their fixes
```

### Examples

#### String Operations
```bash
$ cargo run -- basic strings

=== String Operations Demo ===

1. SET and GET:
   SET message 'Hello, Redis!'
   GET message => 'Hello, Redis!'

2. SET with expiration (EX):
   SET temp_key 'This will expire' EX 5
   TTL temp_key => 5 seconds
...
```

#### Message Queue Pattern
```bash
$ cargo run -- basic lists

=== List Operations Demo ===

7. Message Queue Pattern:
   Producer adding tasks:
     Added task-1
     Added task-2
   Consumer processing tasks:
     Processed: task-1
     Processed: task-2
...
```

## Architecture

The project follows a modular architecture:

```
src/
├── cli/            # Command-line interface
├── demos/          # Redis feature demonstrations
├── models/         # Data models
└── utils/          # Redis client and error handling
```

### Key Components

- **Async-First**: All operations use Tokio for async/await
- **Type-Safe**: Strong typing with comprehensive error handling
- **Modular**: Each Redis feature is a separate, runnable demonstration
- **Educational**: Includes demos for common Rust errors and fixes

## Development

### Running Tests
```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_string_operations
```

### Code Quality
```bash
# Format code
cargo fmt

# Run linter
cargo clippy -- -D warnings

# Check types
cargo check
```

### Building Documentation
```bash
cargo doc --open
```

## Project Structure

```
redis-rust-demo/
├── Cargo.toml              # Project dependencies
├── README.md               # This file
├── CLAUDE.md               # AI assistant instructions
├── RUST.md                 # Rust best practices guide
├── docker-compose.yml      # Redis container setup
├── src/
│   ├── main.rs            # Application entry point
│   ├── lib.rs             # Library root
│   ├── cli/               # CLI implementation
│   ├── demos/             # Feature demonstrations
│   ├── models/            # Data structures
│   └── utils/             # Utilities
└── tests/
    └── integration_tests.rs # Integration tests
```

## Learning Resources

### Included Documentation
- **RUST.md**: Comprehensive guide to common Rust errors and best practices
- **redis-rust-demo-spec.md**: Complete project specification
- **CLAUDE.md**: Development guidelines and project conventions

### Interactive Learning
The `rust-errors` command demonstrates common Rust pitfalls:
- Ownership and borrowing errors
- Lifetime issues
- Type system challenges
- Async/await patterns
- Performance optimizations

## Redis Patterns Demonstrated

### Caching
- Cache-aside pattern
- TTL management
- Cache invalidation

### Data Structures
- **Lists**: Message queues, activity feeds
- **Sets**: Unique visitors, tags, recommendations
- **Hashes**: User profiles, shopping carts, sessions

### Real-World Use Cases
- E-commerce shopping carts
- Visitor tracking
- Message queue implementation
- User session management

## Contributing

Contributions are welcome! Please ensure:
- All tests pass (`cargo test`)
- Code is formatted (`cargo fmt`)
- No clippy warnings (`cargo clippy`)
- New features include tests

## Performance

The application is optimized for:
- Async operations with Tokio
- Connection pooling with r2d2
- Efficient Redis command usage
- Minimal memory allocations

## Docker Support

Redis and Redis Commander are configured in `docker-compose.yml`:

```bash
# Start services
docker-compose up -d

# View Redis Commander UI
open http://localhost:8081

# Stop services
docker-compose down
```

## Dependencies

- **redis**: Official Redis client for Rust
- **tokio**: Async runtime
- **clap**: Command-line argument parsing
- **serde**: Serialization framework
- **tracing**: Structured logging
- **thiserror**: Error handling

## License

This project is licensed under the Apache License, Version 2.0 - see the [LICENSE](LICENSE) file for details.

Copyright 2025 Robert L. Bergman

## Acknowledgments

- Built with the official [redis-rs](https://github.com/redis-rs/redis-rs) client
- Async powered by [Tokio](https://tokio.rs/)
- CLI created with [clap](https://github.com/clap-rs/clap)

## Status

This project is actively maintained and new features are being added regularly. Check the [specification](redis-rust-demo-spec.md) for the complete roadmap.