
use std::sync::{Mutex, Arc};
use std::thread;
use std::iter::Iterator;

fn main() {
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
