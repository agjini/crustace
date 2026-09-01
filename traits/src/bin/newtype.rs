use std::fmt;
use std::ops::Deref;

// struct Operation(Vec<String>);
//
// impl fmt::Display for Operation {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "'{}'", self.0.join(" + "))
//     }
// }
//
// impl From<Vec<String>> for Operation {
//     fn from(value: Vec<String>) -> Self {
//         Operation(value)
//     }
// }
//
// impl Deref for Operation {
//     type Target = Vec<String>;
//
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
//
// fn main() {
//     let w = vec!["hello".into(), "world".into()].into();
//     println!("w = {w}");
//     println!("w = {}", simple_plan2(&w));
//     println!("w = {}", simple_plan(&w));
//     println!("w = {}", simple_plan(&w.deref().deref()));
// }
//
// fn simple_plan2(o: &Operation) -> i32 {
//     44
// }
//
// fn simple_plan(v: &[String]) -> i32 {
//     v.len() as i32
// }
//

fn add_one(x: i32) -> i32 {
    x + 1
}

fn square(x: i32) -> i32 {
    x * x
}

// fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
//     f(arg) + f(arg)
// }

fn do_twice<T: Fn(i32) -> i32>(f: T, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {answer}");
    let pipo = 3;
    let answer = do_twice(|x| x * pipo, 5);

    println!("The answer is: {answer}");
}
