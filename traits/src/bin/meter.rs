use std::fmt::Display;
use std::ops::Add;

struct Millimeters(u32);

#[derive(Debug)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

impl Add<Millimeters> for Meters {
    type Output = Millimeters;

    fn add(self, other: Millimeters) -> Millimeters {
        Millimeters(self.0 * 1000 + other.0)
    }
}

impl Add for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Millimeters) -> Millimeters {
        Millimeters(self.0 + other.0)
    }
}

impl Display for Meters {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}m", self.0)
    }
}

impl Display for Millimeters {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}mm", self.0)
    }
}
fn main() {
    let meter = Meters(2);
    let mili = Millimeters(8);
    let mili2 = Millimeters(15);

    // let result = mili + meter;
    let result = meter + mili + mili2;
    println!("mesure {}", result);
}
