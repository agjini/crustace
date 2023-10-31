fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    // https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#mutable-references
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
