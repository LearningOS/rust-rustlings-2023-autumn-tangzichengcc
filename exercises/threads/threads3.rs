// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.


use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}
fn send_tx(q: Arc<Mutex<Queue>>, tx: Arc<Mutex<mpsc::Sender<u32>>>) {
    let qc = Arc::clone(&q);
    let qc1 = Arc::clone(&q);

    let tx1 = Arc::clone(&tx);  // Clone tx for the first thread
    let tx2 = Arc::clone(&tx);  // Clone tx for the second thread

    thread::spawn(move || {
        for val in &qc.lock().unwrap().first_half {
            println!("sending {:?}", val);
            tx1.lock().unwrap().send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        for val in &qc1.lock().unwrap().second_half {
            println!("sending {:?}", val);
            tx2.lock().unwrap().send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
}



fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Arc::new(Mutex::new(Queue::new()));
    let queue_length = queue.lock().unwrap().length;
    let tx = Arc::new(Mutex::new(tx));  // Wrap the Sender in an Arc<Mutex>

    send_tx(Arc::clone(&queue), Arc::clone(&tx));  // Pass the wrapped Sender

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
        if total_received == queue_length {
            break;
        }
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length);
}
