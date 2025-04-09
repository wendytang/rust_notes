# Rust Logging Examples

This repository contains examples demonstrating various logging patterns in Rust using the `log` and `env_logger` crates.

## Running Examples

There are two ways to run the code:

1. Run the main example:
```bash
cd logging
RUST_LOG=trace cargo run
```

2. Run specific examples:
```bash
cd logging
RUST_LOG=info cargo run --example custom_logger    # Demonstrates custom logger configuration
```

## Available Examples

### Main Example (`src/main.rs`)
- Demonstrates basic logging setup
- Shows different log levels
- Includes environment variable configuration

### Custom Logger (`examples/custom_logger.rs`)
- Shows custom logger configuration
- Demonstrates different log levels
- Includes environment customization

To run any specific example, use:
```bash
RUST_LOG=<level> cargo run --example <example_name>
```

Where:
- `<level>` is one of: trace, debug, info, warn, error
- `<example_name>` is one of:
  - `custom_logger`

## Log Levels

Available log levels (from highest to lowest priority):
1. Error
2. Warn
3. Info
4. Debug
5. Trace

## Code Structure
```
logging/
├── Cargo.toml
├── src/
│   └── main.rs              # Main logging example
└── examples/
    └── custom_logger.rs     # Custom logger example
```