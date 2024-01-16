use std::collections::HashMap;

fn main() {
    let mut integers = vec![1, 2, 7, 4, 7, 11, 7, 9, 6, 2];
    integers.sort();
    let median = integers[integers.len() / 2 - 1];
    println!("{:?}", median);

    let mut correspondancy = HashMap::new();
    for integer in integers {
        let occurrence = correspondancy.entry(integer).or_insert(0);
        *occurrence += 1;
    }
    println!("{:?}", correspondancy);

    let mut max = 0;
    let mut max_key: Option<i32> = None;
    for (k, v) in correspondancy {
        if v > max {
            max = v;
            max_key = Some(k);
        }
    }
    println!("mode is {:?}", max_key);
}
