use std::ops::Range;

// enum Member {
//     Parent,
//     Child,
// }

// struct Family {}

struct Counter {
    range: Range<usize>,
    value: isize,
}
impl Counter {
    fn new(range: Range<usize>) -> Counter {
        println!("kikou");
        let value = if range.start == 0 {
            -1
        } else {
            range.start as isize - 1
        };
        Counter { range, value }
    }
}

fn main() {
    let counter = Counter::new(0..isize::MAX as usize);

    for i in counter {
        println!("{}", i);
    }
}

// struct CocoError;

// type CocoResult<T> = Result<T, CocoError>;

// impl Iterator for Family {
//     type Item = Member;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         None
//     }
// }

impl Iterator for Counter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        // println!("kikou {} {}", self.value, self.range.end as isize - 1);

        if self.value < self.range.end as isize - 1 {
            self.value += 1;
            Some(self.value as usize)
        } else {
            println!("ta    dam");
            None
        }
    }
}
