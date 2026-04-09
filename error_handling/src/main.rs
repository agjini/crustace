use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Pointer};
use std::io::{ErrorKind, Read};
use std::{fs, io};

struct OurError {
    kind: ErrorKind,
    message: String,
}

impl From<io::Error> for OurError {
    fn from(error: io::Error) -> OurError {
        OurError {
            kind: error.kind(),
            message: "An error occurred".to_string(),
        }
    }
}

impl Debug for OurError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("OutError")
    }
}

impl Display for OurError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Not erreur")
    }
}

impl Error for OurError {}

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = read_username_from_file()?;
    let last_char = last_char_of_first_line(&greeting_file)?;
    println!("Last character of the first line: {}", last_char);
    Ok(())
}

fn read_username_from_file() -> io::Result<String> {
    fs::read_to_string("hello.txt")
}

fn last_char_of_first_line(text: &str) -> Result<char, OurError> {
    text.lines()
        .next()
        .ok_or(OurError {
            kind: ErrorKind::InvalidData,
            message: "No line".to_string(),
        })?
        .chars()
        .last()
        .ok_or(OurError {
            kind: ErrorKind::InvalidData,
            message: "No last character found".to_string(),
        })
}
