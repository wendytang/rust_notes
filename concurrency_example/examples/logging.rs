use log::{error, warn, info, debug, trace};
use std::env;

fn main() {
    // Set the RUST_LOG environment variable if not set
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "debug");
    }
    
    // Initialize the logger
    env_logger::init();
    
    println!("1. This is a println! message - always shows up");
    
    // Different log levels
    error!("2. This is an error! Very bad!");
    warn!("3. This is a warning! Be careful!");
    info!("4. This is info! Good to know!");
    debug!("5. This is debug! Helpful for debugging!");
    trace!("6. This is trace! Very detailed!");
    
    // Debug macro examples
    let x = 42;
    dbg!(x);  // prints file:line = 42
    
    // Debug assertions (only in debug builds)
    debug_assert!(x > 0, "x must be positive");
    
    // Print in debug mode only
    #[cfg(debug_assertions)]
    println!("7. This only prints in debug mode");
    
    // Using format strings
    let name = "Rust";
    info!("8. Formatted message: {}", name);
    debug!("9. Complex format: {name} with number {x}");
    
    // Conditional compilation with cfg
    #[cfg(debug_assertions)]
    debug!("10. This debug message only exists in debug builds");
    
    #[cfg(not(debug_assertions))]
    info!("11. This message only exists in release builds");
}