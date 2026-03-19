struct Point {
    x: i32,
    y: i32,
}

// struct PointTuple(i32, i32);

use Message::{ChangeColor, Move, Quit, Write};

pub enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn main() {
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    /////////////////////

    let msg = ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Quit => quit(),
        Move { x, y } => move_caret(x, y),
        Write(text) => write(text),
        ChangeColor(color) => change_color(color),
    }

    // let x = 'c';
    //
    // match x {
    //     'a'..='j' => println!("early ASCII letter"),
    //     'k'..='z' => println!("late ASCII letter"),
    //     _ => println!("something else"),
    // }
    //
    // let p = Point { x: 0, y: 7 };
    //
    // let Point { y, x: 5 } = p else {
    //     return;
    // };
    //
    // //assert_eq!(0, x);
    // assert_eq!(7, y);
    //
    // let p = PointTuple(0, 7);
    //
    // let PointTuple(5, y) = p else {
    //     return;
    // };
    //
    // //assert_eq!(0, x);
    // assert_eq!(7, y);
    //
    // let x = Some(5);
    // let y = 10;
    //
    // match x {
    //     Some(50..=60) => println!("Got 50 or 60"),
    //     Some(y) => println!("Matched, y = {y}"),
    //     _ => println!("Default case, x = {x:?}"),
    // }
    //
    // println!("at the end: x = {x:?}, y = {y}");
    //
    // let d1 = DateTime::<Utc>::from_naive_utc_and_offset(
    //     NaiveDate::from_ymd_opt(2020, 5, 1)
    //         .unwrap()
    //         .and_hms_opt(12, 0, 0)
    //         .unwrap(),
    //     Utc,
    // );
    //
    // let d2 = DateTime::<Utc>::from_naive_utc_and_offset(
    //     NaiveDate::from_ymd_opt(2018, 5, 1)
    //         .unwrap()
    //         .and_hms_opt(12, 0, 0)
    //         .unwrap(),
    //     Utc,
    // );
    //
    // let r = d1..=d2;
    //
    // println!("{:?}", r);
}

fn write(text: String) {
    println!("Text message: {text}")
}

fn move_caret(x: i32, y: i32) {
    println!("Move in the x direction {x} and in the y direction {y}")
}

fn quit() {
    println!("The Quit variant has no data to destructure.")
}

fn change_color(color: Color) {
    match color {
        Color::Rgb(r, g, b) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Color::Hsv(h, s, v) => {
            println!("Change color to hue {h}, saturation {s}, value {v}");
        }
    }
}
