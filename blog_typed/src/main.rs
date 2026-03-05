use blog_typed::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
    println!("{}", post.content());
}

fn test(o: Option<Post>) -> i32 {
    let Some(p) = o else {
        return 0;
    };

    p.content().len() as i32
}
