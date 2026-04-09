use blog::Post;

fn main() {
    // let array: &[u8] = "sdsdsdd" as  &[u8];
    // let  r: * const u8 = array as * const u8;
    //
    // println!("'{:?}'", unsafe {*r});
    // println!("Hello, world!, {:?}", r);

    afficher_le_total();

    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}

fn afficher_le_total() {


    let total = unsafe {
        let a = "sdsdf".to_string();
        let b = 23;

        println!("{:?}", a);
        b
    };

    println!("{}", total);

}

fn afficher(s: String) {
    println!("{}", s);
}