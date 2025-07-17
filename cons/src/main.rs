// construct function

enum List {
    Cons(i32, Box<List>),
    // Cons(i32, List),
    Nil,
}

fn main() {
    println!("Hello, world!");
}
