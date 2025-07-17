#[derive(Copy, Clone, Debug)]
struct PaiPai {
    age: i32,
    son: Option<PaiPai>,
}

fn main() {
    let b = Box::new(5); //heap
    let s = *b;
    println!("b = {s}");

    let b = Box::new(PaiPai {
        age: 18,
        son: Some(PaiPai { age: 0, son: None }),
    }); //heap
    let s = b.clone();
    println!("b = {b:?}");
    println!("s = {s:?}");
}
