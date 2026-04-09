use crate::GuessFormatError::NotInRange;
use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::num::ParseIntError;
use std::str::FromStr;

struct Guess {
    value: u32,
}

enum GuessFormatError {
    ParseIntError,
    NotInRange,
}

impl From<ParseIntError> for GuessFormatError {
    fn from(_: ParseIntError) -> Self {
        GuessFormatError::ParseIntError
    }
}

impl FromStr for Guess {
    type Err = GuessFormatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.parse()?;
        if value < 1 || value > 100 {
            return Err(NotInRange);
        }
        Ok(Guess { value })
    }
}

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        if io::stdin().read_line(&mut guess).is_err() {
            continue;
        }

        let guess: Guess = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number");
                continue;
            }
        };
        println!("You guessed: {:?}", guess.value);

        match guess.value.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Success");
                break;
            }
        }
    }
}
