use std::time::Instant;

fn fibonacci(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

fn main() {
    let n = 40;  // A computationally intensive number
    
    let start = Instant::now();
    let result = fibonacci(n);
    let duration = start.elapsed();
    
    println!("Fibonacci({}) = {}", n, result);
    println!("Time taken: {:?}", duration);
}