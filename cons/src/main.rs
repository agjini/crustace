// construct function

// https://doc.rust-lang.org/book/ch15-02-deref.html#implicit-deref-coercions-with-functions-and-methods

enum List {
    Nil,
    Cons(i32, Box<List>),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> std::ops::Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
fn main() {
    // let x = 5;
    // let y = MyBox::new(x);
    //
    // let test = "toto";
    // let test_to_string = test.to_string();
    // let ref_test = &test_to_string;
    // let ref_test_2: &str = &test_to_string;
    //
    // assert_eq!(5, x);
    // assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}
