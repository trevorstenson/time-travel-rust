# 🔧 Environment Setup

## Tech Stack
- **Language**: Rust 1.80.0+ (latest stable)
- **JavaScript Engine**: deno_core 0.311+ (V8 JavaScript engine bindings)
- **Async Runtime**: tokio (for event loop and async operations)
- **Build System**: Cargo (Rust's package manager)
- **Target**: Cross-platform (Linux, macOS, Windows)

## Prerequisites
- **Rust toolchain** 1.80.0 or later
- **Git** for version control
- **VS Code** (recommended) with Rust Analyzer extension

### Check Prerequisites
```bash
# Check Rust version
cargo --version
# Should show: cargo 1.80.1+ or newer

# Check Rust compiler
rustc --version
# Should show: rustc 1.80.0+ or newer
```

## Installation Steps

### 1. Install Rust (if not already installed)
```bash
# Install Rust via rustup (official installer)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Restart terminal or run:
source ~/.cargo/env

# Update to latest stable
rustup update stable
```

### 2. Create Project Structure
```bash
# Create new Rust binary project
cargo init --bin time_travel_debugger

# Navigate to project directory
cd time_travel_debugger

# Add required dependencies
cargo add deno_core@0.311
cargo add tokio --features=full
cargo add serde --features=derive
cargo add serde_json
cargo add anyhow
```

### 3. Project Dependencies

The `Cargo.toml` should include these dependencies:
```toml
[package]
name = "time_travel_debugger"
version = "0.1.0"
edition = "2021"

[dependencies]
deno_core = "0.311"
tokio = { version = "1.40", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"

[dev-dependencies]
criterion = "0.5"
```

## How to Run

### Development Mode
```bash
# Build and run with help
cargo run -- --help

# Run with specific JavaScript file
cargo run -- src/examples/hello_world.js

# Run with verbose output and statistics
cargo run -- --verbose src/examples/basic_functions.js

# Run with custom configuration
cargo run -- --verbose --max-snapshots 500 src/examples/basic_functions.js

# Debug output from Rust side
RUST_LOG=debug cargo run
```

### Build for Release
```bash
# Build optimized release binary
cargo build --release

# Run release binary
./target/release/time_travel_debugger
```

### Testing
```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

### Development Tools
```bash
# Format code
cargo fmt

# Lint code
cargo clippy

# Check for errors without building
cargo check

# Generate documentation
cargo doc --open
```

## Development Environment

### VS Code Setup
Recommended extensions:
- **rust-analyzer**: Rust language server
- **CodeLLDB**: Debugger for Rust
- **Better TOML**: TOML file support
- **Error Lens**: Inline error display

### Environment Variables
Create a `.env` file in project root:
```bash
# Debug level (trace, debug, info, warn, error)
RUST_LOG=debug

# Performance tuning
RUST_BACKTRACE=1

# Custom configuration
TIME_TRAVEL_DEBUG=true
```

## Troubleshooting

### Common Issues

#### "rustc not found" or old version
```bash
# Update Rust toolchain
rustup update stable
rustup default stable
```

#### Build errors with deno_core
```bash
# Clean and rebuild
cargo clean
cargo build

# Check for conflicting dependencies
cargo tree
```

#### Permission errors on macOS/Linux
```bash
# Make sure cargo is in PATH
echo $PATH | grep -q cargo || echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

#### Windows-specific issues
- Ensure you have Microsoft C++ Build Tools installed
- Use PowerShell or Windows Terminal for best experience

## Performance Considerations

### Build Optimization
```toml
# Add to Cargo.toml for release builds
[profile.release]
lto = true
codegen-units = 1
panic = "abort"
```

### Development Tips
- Use `cargo check` for fast syntax checking
- Use `cargo clippy` for linting
- Use `cargo fmt` to format code
- Enable incremental compilation for faster builds

## Architecture Overview

The time travel debugger will consist of:

1. **JavaScript Execution Engine**: deno_core + V8
2. **State Capture System**: Records execution snapshots
3. **Time Navigation**: Move forward/backward through execution
4. **Debug Interface**: CLI and potential web interface
5. **State Storage**: Efficient storage of execution history

## Next Steps

After environment setup:
1. Initialize basic deno_core runtime
2. Implement basic JavaScript execution
3. Add state capture mechanisms
4. Build time navigation features
5. Create debugging interface

## Resources

- [deno_core Documentation](https://docs.rs/deno_core/latest/deno_core/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Tokio Guide](https://tokio.rs/tokio/tutorial)
- [V8 JavaScript Engine](https://v8.dev/) 