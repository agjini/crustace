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
        user_preference.unwrap_or_else(|| self.most_stocked())
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

    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let th_handle = thread::spawn(move || {
        thread::sleep(Duration::from_secs(5));
        println!("End of thread: {list:?}");
        list.push(4);
        vec![1, 2]
    });
    println!("After spawning thread");
    // let th_handle.join().unwrap();
    println!("end of program:");
}
