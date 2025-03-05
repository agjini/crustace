fn largest<T>(list: &[T]) -> &T
where
    T: PartialOrd,
{
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {result}");

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let char_int = Point { x: 'B', y: 2 };
    let p = Point { x: 5, y: 10 };

    println!("char_int.x = {}", char_int.x());
    println!("char_int.y = {}", char_int.y());

    println!("{}", float.distance_from_origin());
}

struct Point<T, P> {
    x: T,
    y: P,
}
impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    fn ggg(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T, P> Point<T, P> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &P {
        &self.y
    }
}
