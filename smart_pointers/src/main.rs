#[derive(Copy, Clone, Debug)]
struct PaiPai {
    age: i32,
    //son: Option<PaiPai>,
    thunes: [i32; 10_000_000],
}

fn main() {
    let pai = PaiPai {
        age: 18,
        thunes: [0; 10_000_000],
    };
    let pai = Box::new(pai);

    println!("{:?}", pai.age);

    // let b = Box::new(5); //heap
    // let s = *b;
    // println!("b = {s}");
    //
    // let b = Box::new(PaiPai {
    //     age: 18,
    //     son: Some(PaiPai { age: 0, son: None }),
    // }); //heap
    // let s = b.clone();
    // println!("b = {b:?}");
    // println!("s = {s:?}");
}
