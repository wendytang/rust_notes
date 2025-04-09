use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // Create a bounded channel with capacity 1
    let (tx, rx) = mpsc::channel::<String>();

    // Spawn a sender thread
    let sender = thread::spawn(move || {
        let messages = vec!["Hello", "From", "Other", "Thread"];

        for msg in messages {
            // Send each message
            tx.send(msg.to_string()).unwrap();
            println!("Sent: {}", msg);
            
            // Sleep to demonstrate the channel behavior
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Receive messages in the main thread
    for received in rx {
        println!("Got: {}", received);
    }

    // Wait for sender thread to finish
    sender.join().unwrap();
}