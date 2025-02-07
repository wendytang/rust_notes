# Rust Notes and Examples

This repository contains a collection of Rust examples demonstrating various concepts including concurrency, logging, and memory management.

## Project Structure

```
rust_notes/
├── arc_example/            # Arc (Atomic Reference Counting) examples
│   ├── Cargo.toml
│   ├── src/
│   │   └── main.rs        # Main Arc demonstration
│   ├── examples/
│   │   └── simple_arc.rs  # Basic Arc usage
│   └── README.md          # Arc-specific documentation
│
├── concurrency_example/    # Concurrency patterns examples
│   ├── Cargo.toml
│   ├── src/
│   │   └── main.rs        # Channel (mpsc) demonstration
│   ├── examples/
│   │   └── bank_account_mutex.rs  # Mutex usage
│   └── README.md          # Concurrency-specific documentation
│
├── logging/               # Logging examples
│   ├── Cargo.toml
│   ├── src/
│   │   └── main.rs        # Basic logging setup
│   ├── examples/
│   │   └── custom_logger.rs  # Custom logger configuration
│   └── README.md          # Logging-specific documentation
│
└── src/                   # Other examples
    └── error_example.rs   # Error handling patterns
```

## Available Examples

### 1. Arc Example
Demonstrates thread-safe reference counting with `Arc`.
```bash
cd arc_example
cargo run                    # Run main example
cargo run --example simple_arc  # Run simple Arc example
```

### 2. Concurrency Example
Shows various concurrency patterns including channels and mutexes.
```bash
cd concurrency_example
cargo run                           # Run channel example
cargo run --example bank_account_mutex  # Run mutex example
```

### 3. Logging Example
Demonstrates logging setup and configuration.
```bash
cd logging
RUST_LOG=trace cargo run            # Run main logging example
RUST_LOG=info cargo run --example custom_logger  # Run custom logger example
```

### 4. Error Handling Example
Shows error handling patterns.
```bash
cargo run --bin error_example
```

## Key Concepts Covered

1. **Memory Management**
   - Arc (Atomic Reference Counting)
   - Thread-safe memory sharing
   - Reference counting patterns

2. **Concurrency**
   - Channels (mpsc)
   - Mutex synchronization
   - Thread spawning and management
   - Message passing

3. **Logging**
   - Log levels
   - Environment configuration
   - Custom logger setup
   - Structured logging

4. **Error Handling**
   - Error propagation
   - Custom error types
   - Result and Option patterns

## Running Examples

Each subdirectory contains its own examples and documentation. Navigate to the specific directory and follow the instructions in its README.md file.

For example:
```bash
# To run Arc examples:
cd arc_example
cargo run
# or
cargo run --example simple_arc

# To run logging examples with different log levels:
cd logging
RUST_LOG=trace cargo run
```

## Documentation

Each example directory contains:
- A README.md with specific instructions
- Example code with comments
- Multiple examples demonstrating different aspects
- Instructions for running with different configurations

## Contributing

Feel free to add more examples or improve existing ones. Please maintain the same structure:
1. Place new examples in appropriate directories
2. Include clear documentation
3. Add both main.rs and example/ demonstrations
4. Update the relevant README.md files