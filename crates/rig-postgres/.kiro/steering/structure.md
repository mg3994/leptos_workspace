# Project Structure

## Root Directory
```
rig-postgres/
├── .gitignore          # Git ignore patterns (excludes /target)
├── Cargo.toml          # Package manifest and dependencies
└── src/                # Source code directory
    └── lib.rs          # Library root module
```

## Organization Patterns

### Library Structure
- **`src/lib.rs`**: Main library entry point and public API
- **Workspace Member**: This crate is part of a larger workspace, inheriting shared configuration

### Dependency Management
- Uses workspace-level dependency management (`workspace = true`)
- Shared dependencies are defined at the workspace root
- Local dependencies reference other workspace members (e.g., `rig-core`)

### Code Organization Guidelines
- Keep the main library interface in `src/lib.rs`
- Use modules for logical separation of functionality
- Follow Rust naming conventions (snake_case for modules, PascalCase for types)
- Organize database-related code by functionality (connections, queries, vector operations)

### Testing Structure
- Unit tests: Use `#[cfg(test)]` modules within source files
- Integration tests: Place in `tests/` directory (if needed)
- Use testcontainers for database integration tests

### Documentation
- Use `///` for public API documentation
- Include examples in doc comments where helpful
- Document async behavior and error conditions
- Reference PostgreSQL/pgvector requirements in module docs