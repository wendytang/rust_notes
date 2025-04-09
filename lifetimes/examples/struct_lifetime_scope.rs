#[derive(Debug)]
struct DifferentLifetimes<'a, 'b> {
    long_lived: &'a str,
    short_lived: &'b str,
}

fn main() {
    let long_string = String::from("I live long");
    let diff_lifetimes; // Try to store it here
    
    {
        let short_string = String::from("I live short");
        diff_lifetimes = DifferentLifetimes {
            long_lived: &long_string,
            short_lived: &short_string,
        };
        
        // This works fine - both references are still valid
        println!("Inside block - Long: {}, Short: {}", 
                diff_lifetimes.long_lived, 
                diff_lifetimes.short_lived);
    } // short_string is dropped here
    
    // This would fail - can't use diff_lifetimes here
    // println!("Outside block - Long: {}", diff_lifetimes.long_lived);
    // Even though long_lived is still valid, the struct instance
    // cannot outlive its shortest lifetime ('b from short_string)
}