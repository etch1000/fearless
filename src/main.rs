use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        for i in 1..10 {
            println!("Hi number {} from the spawned thread!", i);

            println!("And here is a vector : {:?}", v);

            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);

        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    let (tx, rx) = mpsc::channel();

    let tx_1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx_1.send(val).unwrap();

            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let more_vals = vec![
            String::from("more"),
            String::from("values"),
            String::from("another"),
            String::from("thread"),
        ];

        for val in more_vals {
            tx.send(val).unwrap();

            thread::sleep(Duration::from_secs(1));
        }
    });

    for rcvd in rx {
        println!("Got: {}", rcvd);
    }

    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result : {}", *counter.lock().unwrap());
}
