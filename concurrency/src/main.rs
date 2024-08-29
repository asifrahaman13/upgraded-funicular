use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    // Create a channel for the data transfer.

    let (tx, rx) = mpsc::channel();
    let multi_threading = thread::spawn(move || {
        for index in 1..12 {
            println!("Hi this is a thread {}", index);
            thread::sleep(Duration::from_millis(1));
            tx.send("meaningful message from threading").unwrap();
        }

        let vector_values = vec!["first value", "second value", "nice values"];
        for val in vector_values {
            tx.send(val).unwrap();
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    multi_threading.join().unwrap();

    // let received=rx.recv().unwrap();
    // println!("The received value is {}", received)

    for received_values in rx {
        println!("The received value is {}", received_values);
    }
}
