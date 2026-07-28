use decimal::d128;
use std::ops::{AddAssign, Range};
// enum Member {
//     Parent,
//     Child,
// }

// struct Family {}

struct Counter<T: Copy> {
    range: Range<T>,
    value: T,
    step: T,
}

impl<T: Copy> Counter<T> {
    fn new(range: Range<T>, step: T) -> Counter<T> {
        println!("kikou");
        let value = range.start;
        Counter { range, value, step }
    }
}

struct CocoError;

type CocoResult<T> = Result<T, CocoError>;

impl<T> Iterator for Counter<T>
where
    // trait bounds
    T: Copy + PartialOrd + AddAssign,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.value < self.range.end {
            let value_to_return = self.value;
            self.value += self.step;
            Some(value_to_return)
        } else {
            None
        }
    }
}

trait Itete<T> {
    fn next(&mut self) -> Option<T>;
}

// impl Itete<i32> for Counter<i32> {
//     fn next(&mut self) -> Option<i32> {
//         Some(4)
//     }
// }
//
//
// impl Itete<f32> for Counter<i32> {
//     fn next(&mut self) -> Option<f32> {
//         Some(44.678)
//     }
// }

use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<i32> for Point {
    type Output = Point;

    fn add(self, other: i32) -> Self::Output {
        Point {
            x: self.x + other,
            y: self.y + other,
        }
    }
}

impl AddAssign<i32> for Point {
    fn add_assign(&mut self, rhs: i32) {
        *self = *self + rhs;
    }
}

#[test]
fn test_add() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    let mut pp = Point { x: 1, y: 0 };
    //pp++;
    pp += 1;

    let i = 4;
    let f = --i;

    println!("{:?}", f);

    assert_eq!(Point { x: 10, y: 0 } + 10, Point { x: 20, y: 10 });
}

#[derive(Clone)]
struct Pipo {
    x: usize,
}

const NOTES: [&str; 4] = ["do", "re", "mi", "fa"];
impl Pipo {
    fn new() -> Pipo {
        Pipo { x: 0 }
    }
}

impl Iterator for Pipo {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        let result = NOTES.get(self.x).map(|n| n.to_string());
        self.x += 1;
        result
    }
}

fn main() {
    let pipo = Pipo::new();
    for i in pipo.clone() {
        println!("{}", i);
    }
    for i in pipo {
        println!("{}", i);
    }
    // let counter = Counter::new(10..20, 5);
    // for i in counter {
    //     println!("{}", i);
    // }
    //
    // let counter = Counter::new(d128!(10)..d128!(20), d128!(0.1));
    // for i in counter {
    //     println!("{}", i);
    // }

    //
    // let phrase = "bachibouzouk".to_string();
    // let c = Counter::new(0..phrase.len(), 1);
    // for i in c {
    //     println!("{}", phrase.chars().nth(i).unwrap());
    // }
}
