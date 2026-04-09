use std::thread;
use std::time::Duration;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| Inventory::most_stocked(self))
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    fn expensive_closure(num: u32) -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    }

    expensive_closure(30);

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    // let mut list = vec![1, 2, 3];
    // println!("Before defining closure: {list:?}");

    // let mut borrows_mutably = move || {
    //     list.push(7);
    //     let new_list = list.clone();
    //     list.push(9);
    //     new_list
    // };
    // let mut list = borrows_mutably();
    // println!("After calling closure: {list:?}");
    // list.push(8);
    // let list_2 = borrows_mutably();
    // println!("After calling closure: {list:?}, {list_2:?}");
    //
    // let mut list = vec![1, 2, 3];
    // println!("Before defining closure: {list:?}");
    //
    // let th_handle = thread::spawn(move || {
    //     thread::sleep(Duration::from_secs(5));
    //     println!("End of thread: {list:?}");
    //     list.push(4);
    //     vec![1, 2]
    // });
    // println!("After spawning thread");
    // // let th_handle.join().unwrap();
    // println!("end of program:");
    //

    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut closure = move || {
        println!("From thread: {list:?}");
        list.push(4);
        list.clone()
    };

    // println!("After defining closure {list:?}");
    println!("{:?}", closure());
    let list = closure();
    closure();

    // let closure1 = move || {
    //     println!("From thread: {list:?}");
    // };

    //println!("After defining closure {list:?}");

    let closure2f = move || {
        println!("From thread: {list:?}");
        list
    };

    let list_returned = closure2f();
    closure2f();

    let mut list = vec![1, 2, 3];
    let closure3 = move || {
        println!("From thread: {list:?}");
        list.push(4);
        list
    };

    let mut list = vec![1, 2, 3];
    let mut closure4 = || {
        println!("From thread: {list:?}");
        list.push(4);
    };

    //closure4();

    closure4();
    closure4();
    closure4();
    closure4();
    closure4();
    closure4();
    closure4();

    //thread::spawn(closure3);

    // drop list

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    list.sort_by_key(|r| r.width);
    println!("{list:#?}");

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    // let mut sort_operations = vec![];
    // let value = String::from("closure called");
    //
    // print!("{sort_operations:#?}");
    //
    // let close = |r: &Rectangle| {
    //     sort_operations.push(String::from("closure called"));
    //     r.width
    // };
    //
    // list.sort_by_key(close);
    //
    // print!("{sort_operations:#?}");
    // println!("{list:#?}");
    // let mut sort_operations = vec![];
    // let value = String::from("closure called");

    let mut num_sort_operations = 0;

    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{list:#?}, sorted in {num_sort_operations} operations");
}
