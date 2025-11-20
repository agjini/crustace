use std::sync::Arc;
use std::thread;

mod counter;

use counter::Counter;

fn main() {
    let counter = Counter::new(0);
    let table = Arc::new(vec![0; 10]);
    let mut handles = vec![];

    for _ in 0..10 {
        let mut counter = Counter::clone(&counter);
        let table = Arc::clone(&table);
        let handle = thread::spawn(move || {
            counter.inc();
            println!("table {:?}", table);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", counter.value());
}
