// Example 1: Static lifetime - useful for constants and string literals
struct Config {
    app_name: &'static str,       // String literal - lives for entire program
    version: &'static str,        // String literal - lives for entire program
}

// Example 2: Owned String - useful for dynamic content
struct User {
    name: String,                 // Owned - can be any string, even dynamic
}

// Example 3: When static lifetime is useful - global configuration
static APP_NAME: &str = "MyApp";  // Implicit 'static
static VERSION: &str = "1.0.0";   // Implicit 'static

fn main() {
    // Example 1: Using static strings
    let config = Config {
        app_name: "MyApp",        // String literal - automatically 'static
        version: "1.0.0",         // String literal - automatically 'static
    };
    
    // Example 2: Dynamic string that can't be static
    let username = format!("user_{}", 123);
    let user = User {
        name: username,           // username is moved here
    };
    // println!("{}", username);  // Error! username was moved
    
    // Example 3: When to use 'static - Configuration and Constants
    let config = Config {
        app_name: APP_NAME,       // References to static data
        version: VERSION,
    };
    
    // Example 4: Common use case for 'static - Error messages
    fn get_error() -> &'static str {
        "An error occurred"       // String literal lives for entire program
    }
    
    // Example 5: Why not to use 'static for dynamic data
    let dynamic_string = String::from("dynamic");
    // let config_wrong = Config {
    //     app_name: &dynamic_string,  // Error! Not static
    //     version: "1.0.0",
    // };
    
    println!("Config: {} {}", config.app_name, config.version);
    println!("User: {}", user.name);
    println!("Error: {}", get_error());
}