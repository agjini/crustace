fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, item) in bytes.iter().enumerate() {
        println!("{i}/{item}");
        if *item == b' ' {
            return i;
        }
    }

    s.len()

    // https://doc.rust-lang.org/book/ch04-03-slices.html#the-slice-type
}

fn main(){
    let s = String::from("hello world");
    first_word(&s);
}
