use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    // Sender thread that will be dropped
    let sender1 = thread::spawn(move || {
        // Send a few messages
        tx.send("Message 1").unwrap();
        tx.send("Message 2").unwrap();
        
        // Channel will be closed when tx is dropped at end of thread
    });

    // Wait a bit to ensure some messages are sent
    thread::sleep(Duration::from_millis(100));

    // Receiving with error handling
    loop {
        match rx.recv() {
            Ok(msg) => println!("Received: {}", msg),
            Err(e) => {
                println!("Channel closed: {}", e);
                break;
            }
        }
    }

    // Try receiving with timeout
    match rx.recv_timeout(Duration::from_secs(1)) {
        Ok(msg) => println!("Received message: {}", msg),
        Err(mpsc::RecvTimeoutError::Timeout) => println!("Timeout occurred!"),
        Err(mpsc::RecvTimeoutError::Disconnected) => println!("Channel is disconnected!"),
    }

    // Try sending to a closed channel
    let (tx, rx) = mpsc::channel();
    drop(rx); // Close the channel by dropping the receiver

    match tx.send("test") {
        Ok(_) => println!("Message sent"),
        Err(e) => println!("Failed to send: {}", e),
    }

    sender1.join().unwrap();
}