use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("=== Tokio MPSC Channel Example ===\n");

    // Create a bounded channel with capacity 32
    let (tx, mut rx) = mpsc::channel(32);
    
    // Clone the sender for multiple producers
    let tx2 = tx.clone();

    // First producer task
    let producer1 = tokio::spawn(async move {
        for i in 1..=3 {
            tx.send(format!("Producer 1: Message {}", i)).await.unwrap();
            sleep(Duration::from_millis(100)).await;
        }
    });

    // Second producer task
    let producer2 = tokio::spawn(async move {
        for i in 1..=3 {
            tx2.send(format!("Producer 2: Message {}", i)).await.unwrap();
            sleep(Duration::from_millis(150)).await;
        }
    });

    // Receiver task (could also be in the main task)
    let receiver = tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            println!("Received: {}", message);
        }
    });

    // Wait for all tasks to complete
    producer1.await.unwrap();
    producer2.await.unwrap();
    receiver.await.unwrap();

    println!("\n=== Broadcast Channel Example ===\n");
    
    // Create a broadcast channel
    let (tx, mut rx1) = tokio::sync::broadcast::channel(16);
    let mut rx2 = tx.subscribe(); // Create another receiver

    // Sender task
    let broadcast_sender = tokio::spawn({
        let tx = tx.clone();
        async move {
            for i in 1..=3 {
                tx.send(format!("Broadcast message {}", i)).unwrap();
                sleep(Duration::from_millis(100)).await;
            }
        }
    });

    // First receiver task
    let broadcast_receiver1 = tokio::spawn(async move {
        while let Ok(msg) = rx1.recv().await {
            println!("Receiver 1 got: {}", msg);
        }
    });

    // Second receiver task
    let broadcast_receiver2 = tokio::spawn(async move {
        while let Ok(msg) = rx2.recv().await {
            println!("Receiver 2 got: {}", msg);
        }
    });

    // Wait for broadcast tasks
    broadcast_sender.await.unwrap();
    sleep(Duration::from_millis(500)).await; // Give receivers time to process
    
    println!("\n=== Watch Channel Example ===\n");

    // Create a watch channel with String type
    let (tx, mut rx) = tokio::sync::watch::channel(String::from("initial value"));

    // Sender task
    let watch_sender = tokio::spawn(async move {
        for i in 1..=3 {
            tx.send(format!("Watch value {}", i)).unwrap();
            sleep(Duration::from_millis(100)).await;
        }
    });

    // Receiver task
    let watch_receiver = tokio::spawn(async move {
        while rx.changed().await.is_ok() {
            println!("Watch value changed to: {}", *rx.borrow());
        }
    });

    // Wait for watch tasks
    watch_sender.await.unwrap();
    watch_receiver.await.unwrap();
}