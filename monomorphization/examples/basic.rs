fn process<T>(x: T) -> T {
    x
}

fn main() {
    let int_result = process::<i32>(42);
    let float_result = process::<f64>(3.14);
    let string_result = process::<&str>("hello");

    // Using turbofish syntax
    println!("int: {}", int_result);
    println!("float: {}", float_result);
    println!("string: {}", string_result);

    // Type inference (no turbofish needed)
    let inferred_int = process(42);        // T is inferred as i32
    let inferred_float = process(3.14);    // T is inferred as f64
    let inferred_string = process("hello"); // T is inferred as &str
}