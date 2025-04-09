use std::fmt::Debug;

// Generic struct with two type parameters
struct Pair<T, U> {
    first: T,
    second: U,
}

// Generic implementation with constraint
impl<T: Debug, U: Debug> Pair<T, U> {
    fn print_types(&self) {
        println!("First type: {:?}, Second type: {:?}", 
                std::any::type_name::<T>(), 
                std::any::type_name::<U>());
    }
}

fn main() {
    // Different concrete types
    let pair1: Pair<i32, String> = Pair {
        first: 42,
        second: String::from("hello"),
    };

    let pair2: Pair<bool, f64> = Pair {
        first: true,
        second: 3.14,
    };

    pair1.print_types();
    pair2.print_types();
}