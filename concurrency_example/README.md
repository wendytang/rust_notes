# Rust Concurrency Examples

This repository contains examples demonstrating various concurrency patterns in Rust.

## Running Examples

There are two ways to run the code:

1. Run the main example:
```bash
cd concurrency_example
cargo run
```

2. Run specific examples:
```bash
cd concurrency_example
cargo run --example bank_account_mutex    # Demonstrates Mutex usage
```

## Available Examples

### Main Example (`src/main.rs`)
- Demonstrates basic channel (mpsc) usage
- Shows multiple producer, single consumer pattern
- Includes thread spawning and message passing

### Bank Account with Mutex (`examples/bank_account_mutex.rs`)
- Shows thread-safe state management using Mutex
- Demonstrates concurrent access to shared data
- Includes proper synchronization techniques

To run any specific example, use:
```bash
cargo run --example <example_name>
```

Where `<example_name>` is one of:
- `bank_account_mutex`

## Key Concepts

1. Channels (mpsc)
   - Multiple producer, single consumer pattern
   - Safe communication between threads
   - Built-in synchronization

2. Mutex
   - Safe shared state between threads
   - Lock-based synchronization
   - Preventing data races

3. Threads
   - Spawning new threads
   - Moving values into threads
   - Thread synchronization

## Code Structure
```
concurrency_example/
├── Cargo.toml
├── src/
│   └── main.rs           # Main channel example
└── examples/
    └── bank_account_mutex.rs  # Mutex example
```