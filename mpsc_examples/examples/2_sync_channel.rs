use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // Create a synchronous channel with capacity 2
    let (tx, rx) = mpsc::sync_channel(2);

    let sender = thread::spawn(move || {
        for i in 1..=5 {
            println!("Sending message {}...", i);
            // This will block if the channel is full
            tx.send(i).unwrap();
            println!("Message {} sent!", i);
        }
    });

    // Simulate slow receiver to demonstrate blocking
    for received in rx {
        println!("Received: {}", received);
        // Sleep to simulate slow processing
        thread::sleep(Duration::from_secs(1));
    }

    sender.join().unwrap();
}

// Key differences from regular channel:
// 1. sync_channel blocks when full
// 2. Regular channel (channel()) would buffer all messages
// 3. sync_channel provides backpressure