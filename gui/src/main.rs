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
//
// impl Draw for PrettyString {
//     fn draw(&self) {
//         println!("************");
//         println!("{}", self.string);
//         println!("************");
//     }
// }

fn main() {
    let screen = Screen::new()
        .add(PrettyString {
            string: "Hello World!".to_string(),
        })
        .add(Button {
            width: 10,
            height: 20,
            label: "Click me!".to_string(),
        })
        .add(PrettyButton {
            button: Button {
                width: 10,
                height: 20,
                label: "Click me!".to_string(),
            },
        })
        .add(SelectBox {
            width: 10,
            height: 20,
            options: vec!["aaa".to_string(), "bbb".to_string()],
        })
        .build();

    screen.run();
}

impl Draw for PrettyString {
    fn draw(&self) {
        println!("************");
        println!("{}", self.string);
        println!("************");
    }
}
