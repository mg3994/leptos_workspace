# Technology Stack

## Build System
- **Cargo**: Rust's package manager and build tool
- **Workspace**: Part of a larger Rust workspace (uses `workspace = true` for shared dependencies)

## Core Technologies
- **Rust**: Primary programming language (2021 edition)
- **PostgreSQL**: Database backend with pgvector extension
- **Tokio**: Async runtime for concurrent operations

## Key Dependencies
- `rig-core`: Core framework library with derive features
- `sqlx`: Async PostgreSQL driver with JSON and UUID support
- `pgvector`: PostgreSQL vector extension bindings
- `serde`: Serialization/deserialization with JSON support
- `tracing`: Structured logging and instrumentation
- `uuid`: UUID generation and handling

## Development Dependencies
- `tokio`: Multi-threaded async runtime for tests
- `testcontainers`: Docker-based integration testing
- `httpmock`: HTTP mocking for tests
- `tracing-subscriber`: Log output formatting
- `dotenvy`: Environment variable loading

## Common Commands

### Building
```bash
cargo build                 # Build the library
cargo build --release      # Release build
```

### Testing
```bash
cargo test                  # Run all tests
cargo test --lib           # Run library tests only
```

### Development
```bash
cargo check                 # Fast syntax/type checking
cargo clippy               # Linting
cargo fmt                  # Code formatting
```

### Documentation
```bash
cargo doc --open           # Generate and open documentation
```