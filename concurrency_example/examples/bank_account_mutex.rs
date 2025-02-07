use std::sync::Mutex;
use std::thread;

struct BankAccount {
    balance: Mutex<i32>,
}

fn main() {
    let account = BankAccount {
        balance: Mutex::new(100),
    };

    let mut handles = vec![];

    // Create 5 threads that each withdraw 10
    for _ in 0..5 {
        let handle = thread::spawn(move || {
            let mut balance = account.balance.lock().unwrap();
            *balance -= 10;
            println!("Current balance: {}", *balance);
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final balance: {}", *account.balance.lock().unwrap());
}