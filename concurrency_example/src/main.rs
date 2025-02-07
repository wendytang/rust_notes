use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    // Create a channel
    let (tx, rx) = mpsc::channel();

    // Clone the transmitter for the second thread
    let tx1 = tx.clone();

    // Spawn thread 1
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Spawn thread 2
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Receive messages in the main thread
    for received in rx {
        println!("Got: {}", received);
    }
}