use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // Create an unbounded channel
    let (tx, rx) = mpsc::channel();
    
    // Create multiple producers by cloning the sender
    let tx1 = tx.clone();
    let tx2 = tx.clone();
    let tx3 = tx.clone();

    // Producer 1
    let producer1 = thread::spawn(move || {
        for i in 1..=3 {
            tx1.send(format!("Producer 1: Message {}", i)).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    // Producer 2
    let producer2 = thread::spawn(move || {
        for i in 1..=3 {
            tx2.send(format!("Producer 2: Message {}", i)).unwrap();
            thread::sleep(Duration::from_millis(300));
        }
    });

    // Producer 3
    let producer3 = thread::spawn(move || {
        for i in 1..=3 {
            tx3.send(format!("Producer 3: Message {}", i)).unwrap();
            thread::sleep(Duration::from_millis(700));
        }
    });

    // Drop the original sender
    drop(tx);

    // Consumer
    for received in rx {
        println!("Received: {}", received);
    }

    // Wait for all producers to finish
    producer1.join().unwrap();
    producer2.join().unwrap();
    producer3.join().unwrap();
}