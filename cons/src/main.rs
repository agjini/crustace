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

impl<T: ToString> MyBox<T> {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn print_int(x: &i32) {
    println!("x = {}", x);
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    // let test = "toto";
    // let test_to_string = test.to_string();
    // let ref_test = &test_to_string;
    // let ref_test_2: &str = &test_to_string;
    //
    assert_eq!(5, x);
    assert_eq!(7, *y + 2);

    print_int(&x);
    print_int(&y);

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

#[derive(Debug)]
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        todo!()
    }
}

#[test]
fn test_2() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(c);
    // println!("CustomSmartPointer dropped before the end of main.  {c:?}");
}

fn ddd(_c: CustomSmartPointer) -> CustomSmartPointer {
    _c
}

struct EngineCore;

impl EngineCore {
    fn new() -> EngineCore {
        println!("Engine Core is running.");
        EngineCore
    }

    fn stop(self) {
        println!("Engine Core is stopped.");
    }

    fn do_something(&mut self) {
        println!("Engine Core is doing something.");
    }
}

#[test]
fn test_engine() {
    let mut engine_core = EngineCore::new();
    engine_core.do_something();
    engine_core.stop();
    // engine_core.do_something();
}

struct App {
    engine_core: Option<EngineCore>,
}

impl App {
    fn new() -> App {
        App {
            engine_core: Some(EngineCore::new()),
        }
    }

    fn run(&mut self) {
        // TODO Check the state of the APP
        if let Some(engine_core) = &mut self.engine_core {
            engine_core.do_something();
        }
    }
}

impl Drop for App {
    fn drop(&mut self) {
        if let Some(engine_core) = self.engine_core.take() {
            engine_core.detach();
        }
    }
}

struct NotInitialized {
    config: String,
}

struct Running {
    is_enabled: bool,
}

impl NotInitialized {
    fn to_state2(self) -> Running {
        println!("State1 is stopping.");
        Running {
            is_enabled: self.config,
        }
    }
}

struct Engine<State> {
    state: State,
}

impl Engine<NotInitialized> {
    fn new(config: String) -> Engine<NotInitialized> {
        println!("Engine is starting with config: {}", config);
        Engine {
            state: NotInitialized { config },
        }
    }

    fn run(self) -> Engine<Running> {
        let state2 = self.state.to_state2();
        println!("Engine has stopped.");
        Engine { state: state2 }
    }
}

impl Engine<Running> {
    fn do_something(&self) {
        if self.state.is_enabled {
            println!("Engine is doing something.");
        } else {
            println!("Engine is not enabled.");
        }
    }
}
