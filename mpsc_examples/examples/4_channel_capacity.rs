use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // Example 1: Bounded Channel (capacity = 2)
    println!("=== Bounded Channel Example ===");
    let (tx1, rx1) = mpsc::sync_channel(2);

    let sender1 = thread::spawn(move || {
        for i in 1..=5 {
            println!("Attempting to send {}", i);
            tx1.send(i).unwrap();
            println!("Sent {}", i);
        }
    });

    // Slow receiver to demonstrate blocking
    for received in rx1 {
        println!("Received: {}", received);
        thread::sleep(Duration::from_millis(500));
    }
    sender1.join().unwrap();

    // Example 2: Unbounded Channel
    println!("\n=== Unbounded Channel Example ===");
    let (tx2, rx2) = mpsc::channel();

    let sender2 = thread::spawn(move || {
        for i in 1..=5 {
            println!("Sending {} to unbounded channel", i);
            tx2.send(i).unwrap();
        }
    });

    // Wait a bit to let sender queue up messages
    thread::sleep(Duration::from_millis(100));

    // Process all queued messages
    for received in rx2 {
        println!("Received from unbounded: {}", received);
    }
    sender2.join().unwrap();

    // Example 3: Zero-capacity Channel
    println!("\n=== Zero-capacity Channel Example ===");
    let (tx3, rx3) = mpsc::sync_channel(0);

    let sender3 = thread::spawn(move || {
        for i in 1..=3 {
            println!("Trying to send {} (will block until received)", i);
            tx3.send(i).unwrap();
            println!("Sent {}", i);
        }
    });

    // Receive each message with delay to demonstrate blocking
    for _ in 0..3 {
        thread::sleep(Duration::from_secs(1));
        println!("Received: {}", rx3.recv().unwrap());
    }
    sender3.join().unwrap();
}