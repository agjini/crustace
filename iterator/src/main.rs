fn main() {
    let v1 = vec![1, 2, 3];

    // for i in 0.. v1.len() {
    //     println!("Got: {}", v1[i]);
    // }
    //
    // for val in v1 {
    //     println!("Got: {}", val);
    // }

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {val}");
    }
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2 = v1.iter().map(|x| x + 1).collect::<Vec<_>>();

    assert_eq!(v2, vec![2, 3, 4]);
}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter;
    v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1));

    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}

#[test]
fn iterator_sum2() {
    let v1: Vec<i32> = vec![1, 2, 3];
    //////////////
    let bibi = v1.iter();

    let bibi_map = bibi.map(|x| x + 1);

    for x in bibi_map {
        println!("{x} = {}", x);
    }
    //////////////
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    let toto = shoes.into_iter().filter(|s| s.size == shoe_size);
    toto.collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}
