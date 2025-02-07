use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// Simulated bank account
struct BankAccount {
    balance: i32,
    pending_transactions: i32,
}

fn main() {
    // Create a shared bank account with Mutex for safe concurrent access
    let account = Arc::new(Mutex::new(BankAccount {
        balance: 100,
        pending_transactions: 0,
    }));

    let mut handles = vec![];

    // Spawn multiple deposit threads
    for i in 0..3 {
        let account = Arc::clone(&account);
        let handle = thread::spawn(move || {
            // Simulate some work
            thread::sleep(Duration::from_millis(50));
            
            // Lock the account to modify it
            let mut acc = account.lock().unwrap();
            acc.pending_transactions += 1;
            let amount = 50;
            acc.balance += amount;
            println!("Thread {} deposited {}. New balance: {}", i, amount, acc.balance);
            acc.pending_transactions -= 1;
        });
        handles.push(handle);
    }

    // Spawn multiple withdrawal threads
    for i in 0..2 {
        let account = Arc::clone(&account);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(20));
            
            let mut acc = account.lock().unwrap();
            acc.pending_transactions += 1;
            let amount = 70;
            if acc.balance >= amount {
                acc.balance -= amount;
                println!("Thread {} withdrew {}. New balance: {}", i, amount, acc.balance);
            } else {
                println!("Thread {} failed to withdraw: insufficient funds", i);
            }
            acc.pending_transactions -= 1;
        });
        handles.push(handle);
    }

    // Spawn a monitoring thread
    let account_monitor = Arc::clone(&account);
    handles.push(thread::spawn(move || {
        for i in 0..5 {
            thread::sleep(Duration::from_millis(40));
            let acc = account_monitor.lock().unwrap();
            println!("Monitor {}: Balance = {}, Pending transactions = {}", 
                    i, acc.balance, acc.pending_transactions);
        }
    }));

    // Wait for all operations to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Final account state
    let final_state = account.lock().unwrap();
    println!("\nFinal balance: {}", final_state.balance);
    println!("Final pending transactions: {}", final_state.pending_transactions);
}