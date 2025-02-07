use std::error::Error;
use std::fs::File;
use std::io::Read;

// Without Box<dyn Error>
fn read_config_verbose() -> Result<String, std::io::Error> {
    let mut file = File::open("config.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// With Box<dyn Error>
fn read_config_flexible() -> Result<String, Box<dyn Error>> {
    let mut file = File::open("config.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    // We could also parse JSON here
    if contents.is_empty() {
        return Err("Config file is empty".into()); // String error gets converted automatically
    }
    
    Ok(contents)
}

// Example with multiple error types
fn process_data() -> Result<(), Box<dyn Error>> {
    // Could fail with io::Error
    let data = std::fs::read_to_string("data.txt")?;
    
    // Could fail with ParseIntError
    let number: i32 = data.trim().parse()?;
    
    // Could fail with custom error
    if number < 0 {
        return Err("Number cannot be negative".into());
    }
    
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Example usage
    match process_data() {
        Ok(()) => println!("Processing successful"),
        Err(e) => println!("Error: {}", e),
    }
    
    Ok(())
}