use gui::{Button, Draw, PrettyButton, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

struct PrettyString {
    pub string: String,
}

impl Draw for SelectBox {
    fn draw(&self) {
        for x in self.options.iter() {
            println!("-{}", x);
        }
    }
}

impl Draw for PrettyString {
    fn draw(&self) {
        println!("************");
        println!("{}", self.string);
        println!("************");
    }
}

fn main() {
    let mut screen = Screen {
        components: Vec::new(),
    };
    screen.components.push(Box::new(Button {
        width: 10,
        height: 20,
        label: "Click me!".to_string(),
    }));
    screen.components.push(Box::new(PrettyButton {
        button: Button {
            width: 10,
            height: 20,
            label: "Click me!".to_string(),
        },
    }));
    screen.components.push(Box::new(SelectBox {
        width: 10,
        height: 20,
        options: vec!["aaa".to_string(), "bbb".to_string()],
    }));
    screen.components.push(Box::new(PrettyString {
        string: "Hello World!".to_string(),
    }));
    screen.run();
}
