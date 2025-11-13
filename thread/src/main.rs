use std::sync::atomic::AtomicI32;
use std::sync::Arc;
use std::thread;

fn main() {
    let counter = Arc::new(AtomicI32::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // let mut num = counter.lock().unwrap();

            counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // println!("Result: {}", *counter.lock().unwrap());
    println!("Result: {:?}", *counter);
}
