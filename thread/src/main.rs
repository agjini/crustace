use std::sync::mpsc;
use std::thread;

fn _main2() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

    handle.join().unwrap();
}

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        let val2 = String::from("ho");
        tx.send(val).unwrap();
        tx.send(val2).unwrap();
    });

    let toto = rx.recv().unwrap();
    let tata = rx.recv().unwrap();
    println!("Got: {}", toto);
    println!("Got: {}", tata);
}
