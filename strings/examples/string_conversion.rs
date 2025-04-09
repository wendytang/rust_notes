// Demonstrating various ways to convert between String and str
fn main() {
    // 1. &str to String
    println!("1. Converting &str to String:");
    
    // Using String::from
    let string1 = String::from("hello");
    println!("Using String::from: {}", string1);
    
    // Using to_string()
    let string2 = "hello".to_string();
    println!("Using to_string(): {}", string2);
    
    // Using to_owned()
    let string3 = "hello".to_owned();
    println!("Using to_owned(): {}", string3);

    // Using into()
    let string4: String = "hello".into();
    println!("Using into(): {}", string4);

    // 2. String to &str
    println!("\n2. Converting String to &str:");
    
    let owned = String::from("hello");
    
    // Using &
    let slice1: &str = &owned;
    println!("Using reference: {}", slice1);
    
    // Using as_str()
    let slice2 = owned.as_str();
    println!("Using as_str(): {}", slice2);

    // 3. Conversion in Function Calls
    println!("\n3. Function Call Conversions:");
    
    // Function that takes &str
    fn takes_str(s: &str) {
        println!("Got str: {}", s);
    }

    // Can pass both &str and String (via deref coercion)
    takes_str("literal");  // passing &str directly
    takes_str(&owned);     // passing &String (auto-converts to &str)

    // 4. Working with String in Structs
    println!("\n4. Structs with Strings:");

    // Struct with owned String
    struct Person {
        name: String,
    }

    // Creating with String
    let person1 = Person {
        name: String::from("Alice"),
    };
    println!("Person with String: {}", person1.name);

    // Struct with string reference
    struct PersonRef<'a> {
        name: &'a str,
    }

    // Creating with &str
    let person2 = PersonRef {
        name: "Bob",
    };
    println!("Person with &str: {}", person2.name);

    // 5. Conversion in Collection Types
    println!("\n5. Collections:");
    
    // Vec of Strings
    let strings = vec![
        String::from("hello"),
        "world".to_string(),
    ];
    println!("Vec of Strings: {:?}", strings);

    // Vec of &str
    let str_refs = vec!["hello", "world"];
    println!("Vec of &str: {:?}", str_refs);
}