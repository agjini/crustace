use std::thread;

mod counter;

use counter::Counter;

fn main() {
    let counter = Counter::new(0);
    let mut handles = vec![];

    for _ in 0..10 {
        let mut counter = Counter::clone(&counter);
        let handle = thread::spawn(move || {
            counter.inc();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", counter.value());
}
