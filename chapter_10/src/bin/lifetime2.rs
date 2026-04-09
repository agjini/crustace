fn main() {
    let string1 = String::from("abcd");

    {
        let string2 = String::from("xyz");
        main_2(string1.as_str(), &string2);
    }
}

fn main_2<'a>(x: &'a str, y: &'a str) -> &'a str {
    let _ = longest(x, y);
    x
}

// fn longest<'a, 'b>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() { x } else { y }
// }

fn longest<'a>(x: &'a str, y: &'a str) -> String {
    "really long string".into()
}

fn longest_bis<'a, 'b>(x: &'a str, y: &'b str) -> String {
    "really long string".into()
}

fn longestdd(x: &str) -> &str {
    x
}
