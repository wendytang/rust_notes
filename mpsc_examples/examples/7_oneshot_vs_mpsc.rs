use tokio::sync::oneshot;
use std::sync::mpsc;
use std::thread;
use tokio;

#[tokio::main]
async fn main() {
    println!("=== Oneshot Channel Example ===");
    // Create a oneshot channel
    let (tx_oneshot, rx_oneshot) = oneshot::channel();

    // Oneshot can only send once
    tokio::spawn(async move {
        // Send a single value
        if tx_oneshot.send("Hello from oneshot!").is_err() {
            println!("Receiver dropped");
        }
        
        // Cannot send again - tx_oneshot is consumed after first send
        // This wouldn't compile:
        // tx_oneshot.send("Second message"); // Error!
    });

    // Receive the single value
    match rx_oneshot.await {
        Ok(msg) => println!("Received from oneshot: {}", msg),
        Err(_) => println!("Sender dropped"),
    }

    println!("\n=== MPSC Channel Example ===");
    // Create an mpsc channel
    let (tx_mpsc, rx_mpsc) = mpsc::channel();
    let tx_mpsc_clone = tx_mpsc.clone(); // Can clone sender

    // First sender thread
    thread::spawn(move || {
        tx_mpsc.send("Message 1 from sender 1").unwrap();
        tx_mpsc.send("Message 2 from sender 1").unwrap();
    });

    // Second sender thread
    thread::spawn(move || {
        tx_mpsc_clone.send("Message 1 from sender 2").unwrap();
        tx_mpsc_clone.send("Message 2 from sender 2").unwrap();
    });

    // Receive multiple messages
    for _ in 0..4 {
        match rx_mpsc.recv() {
            Ok(msg) => println!("Received from mpsc: {}", msg),
            Err(_) => println!("All senders dropped"),
        }
    }

    println!("\n=== Key Differences ===");
    println!("1. Oneshot: Single send, single receive");
    println!("2. MPSC: Multiple sends, multiple receives");
    println!("3. Oneshot: Async/await based (tokio)");
    println!("4. MPSC: Thread-based (std)");
    println!("5. Oneshot: Sender consumed after send");
    println!("6. MPSC: Sender can be cloned and reused");
}