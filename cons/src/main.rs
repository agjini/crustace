// construct function

// https://doc.rust-lang.org/book/ch15-02-deref.html#implicit-deref-coercions-with-functions-and-methods

use std::ops::Deref;

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

impl<T> Deref for MyBox<T> {
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

    let b = Box::new(Point { x: 5, y: 10 });

    println!("i.x = {}, i.y = {}", b.x, b.y);
    display_point(&b); // 1 coercion
    display(&b); // 2 x coercion

    let string_slice = "Rust";
    let mut string = String::from(String::from("Augu"));
    string.push_str("Maxime");

    let mm = &string[1..4];

    hello(mm);
}

fn display_point(point: &Point) {
    println!("i.x = {}, i.y = {}", point.x, point.y);
}

fn display(point: &i32) {
    println!("i.x = {}", point);
}

#[derive(Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Deref for Point {
    type Target = i32;

    fn deref(&self) -> &i32 {
        &self.x
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}
