fn main() {
    let mut s = String::from("hello");
    //let mut s = "hello";
    println!("add : {:p}", &s);
    s = "bla".to_string();
    println!("add : {:p}", &s);

    let mut s33 = String::from("hello333");
    s.push_str(", world!hellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohello"); // push_str() appends a literal to a String
    println!("add : {:p}", &s);

    println!("{}", s);

    // https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#variables-and-data-interacting-with-move
}
