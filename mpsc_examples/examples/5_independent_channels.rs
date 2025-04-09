use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // Create Channel 1
    let (tx1, rx1) = mpsc::channel();
    
    // Create Channel 2 (completely independent)
    let (tx2, rx2) = mpsc::channel();

    // Thread 1 - uses Channel 1
    let thread1 = thread::spawn(move || {
        for i in 1..=3 {
            tx1.send(format!("Channel 1: Message {}", i)).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    // Thread 2 - uses Channel 2
    let thread2 = thread::spawn(move || {
        for i in 1..=3 {
            tx2.send(format!("Channel 2: Message {}", i)).unwrap();
            thread::sleep(Duration::from_millis(300));
        }
    });

    // Receiver thread for Channel 1
    let receiver1 = thread::spawn(move || {
        for received in rx1 {
            println!("Receiver 1 got: {}", received);
        }
    });

    // Main thread receives from Channel 2
    for received in rx2 {
        println!("Receiver 2 got: {}", received);
    }

    // Wait for all threads to complete
    thread1.join().unwrap();
    thread2.join().unwrap();
    receiver1.join().unwrap();
}