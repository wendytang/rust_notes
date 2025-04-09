use std::sync::mpsc::{self, sync_channel};
use std::thread;
use std::time::Duration;

fn main() {
    // Channel 1: Unbounded channel carrying strings
    let (tx1, rx1) = mpsc::channel::<String>();

    // Channel 2: Bounded synchronous channel carrying numbers
    let (tx2, rx2) = sync_channel::<i32>(2); // capacity of 2

    // Thread using Channel 1 (strings)
    let thread1 = thread::spawn(move || {
        let messages = vec!["Hello", "World", "From", "Channel", "One"];
        for msg in messages {
            tx1.send(msg.to_string()).unwrap();
            println!("Channel 1 sent: {}", msg);
        }
    });

    // Thread using Channel 2 (numbers)
    let thread2 = thread::spawn(move || {
        for i in 1..=5 {
            println!("Channel 2 trying to send: {}", i);
            tx2.send(i).unwrap(); // This might block due to capacity
            println!("Channel 2 sent: {}", i);
        }
    });

    // Receive from Channel 1 (strings) slowly
    let receiver1 = thread::spawn(move || {
        for received in rx1 {
            println!("Channel 1 received: {}", received);
            thread::sleep(Duration::from_millis(200));
        }
    });

    // Receive from Channel 2 (numbers) very slowly
    for received in rx2 {
        println!("Channel 2 received: {}", received);
        thread::sleep(Duration::from_secs(1)); // Slow receiver to demonstrate blocking
    }

    thread1.join().unwrap();
    thread2.join().unwrap();
    receiver1.join().unwrap();
}