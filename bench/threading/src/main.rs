
use std::sync::{Mutex, Arc};
// Multiple-producer, single-consumer
use std::sync::mpsc;
use std::thread;
use std::iter::Iterator;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();

    thread::spawn(move || {
        tx.send(String::from("Hello,")).unwrap();
        tx.send(String::from("World!")).unwrap();
    });

    thread::spawn(move || {
        tx2.send(String::from("Hello again,")).unwrap();
        tx2.send(String::from("Big World!")).unwrap();
    });

    let h = thread::spawn(move || {
        println!("{}", rx.recv().unwrap());
        println!("{}", rx.recv().unwrap());
        println!("{}", rx.recv().unwrap());
        println!("{}", rx.recv().unwrap());
    });

    h.join().unwrap();

    let v: Vec<usize> = (0..10).collect();
    let counter = Arc::new(Mutex::new(v));
    let mut handles = vec![];

    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut v = counter.lock().unwrap();
            v[i] = 0xbeef;
        });

        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("Final result: {:?}", *counter.lock().unwrap());
}
