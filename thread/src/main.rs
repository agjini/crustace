use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;

fn _main2() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

    handle.join().unwrap();
}

fn main() {
    let mut a = "hello".to_string();

    let c = if true {
        let a = 2;
        a + 33
    } else {
        let a = 20;
        a + 33
    };
    let b = &mut a;

    println!("a : {a}, b: {b}, c: {c}");
    //b.push_str(" world");

    // qqch(&mut a); // copy ref

    // println!("b : {a}");
}

fn qqch(s: &mut String) -> i32 {
    println!("s : {s}");
    s.push_str("papay");
    33;
}

fn main3() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || tt(&tx));

    tx.send(String::from("hello")).unwrap();

    // thread::spawn(|| {
    //     let val = String::from("hi");
    //     let val2 = String::from("ho");
    //     tx.send(val).unwrap();
    //     thread::sleep(std::time::Duration::from_secs(15));
    //     tx.send(val2).unwrap();
    // });

    let toto = rx.recv().unwrap();
    println!("Got: {}", toto);
    let tata = rx.recv().unwrap();
    println!("Got: {}", tata);
}

fn tt(tx: &Sender<String>) {
    let val = String::from("hi");
    let val2 = String::from("ho");
    tx.send(val).unwrap();
    thread::sleep(std::time::Duration::from_secs(15));
    tx.send(val2).unwrap();
}
