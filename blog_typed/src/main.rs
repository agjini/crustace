use blog_typed::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
    println!("{}", post.content());

    test(Some(post));
}

fn test(o: Option<Post>) -> i32 {
    let Some(p) = o else {
        return 0;
    };

    let tab = [1, 2, 3, 4];

    let [first, tail @ .., last] = tab;

    println!("{}, {}", first, last);
    println!("{:?}", tail);

    tab.array_windows()
        .map(|[a, b]| a + b)
        .for_each(|sum| println!("{}", sum));

    p.content().len() as i32
}

fn test2(today_post: Option<Post>) -> i32 {
    if let Some(post) = today_post {
        post.content().len() as i32
    } else {
        0
    }
}
