// Example showing how lifetimes work with structs
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,  // This reference must not outlive 'a
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find '.'");
    
    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };

    println!("Excerpt: {:?}", excerpt);
    println!("Level: {}", excerpt.level());
    println!("Part: {}", excerpt.announce_and_return_part("Hello!"));
}