// Example 1: Struct with a single reference
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
}

// Example 2: Struct with multiple references that must share the same lifetime
#[derive(Debug)]
struct TwoRefs<'a> {
    first: &'a str,
    second: &'a str,  // Both references must live as long as 'a
}

// Example 3: Struct with references that can have different lifetimes
#[derive(Debug)]
struct DifferentLifetimes<'a, 'b> {
    long_lived: &'a str,
    short_lived: &'b str,
}

fn main() {
    // Example 1: Basic struct with reference
    let name = String::from("Alice");
    let person = Person { name: &name };
    println!("Person: {:?}", person);

    // Example 2: Both references must live equally long
    let str1 = String::from("hello");
    {
        let str2 = String::from("world");
        let two_refs = TwoRefs {
            first: &str1,
            second: &str2,
        };
        println!("TwoRefs: {:?}", two_refs);
        // two_refs cannot be used outside this scope because str2 doesn't live long enough
    }

    // Example 3: References can have different lifetimes
    let long_string = String::from("I live long");
    {
        let short_string = String::from("I live short");
        let diff_lifetimes = DifferentLifetimes {
            long_lived: &long_string,
            short_lived: &short_string,
        };
        // Can use both references here
        println!("Long: {}, Short: {}", diff_lifetimes.long_lived, diff_lifetimes.short_lived);
        // Cannot keep diff_lifetimes beyond short_string's lifetime
    }
}