pub trait Draw {
    fn draw(&self) {
        self.default_draw();
    }
    fn default_draw(&self) {
        println!("default_draw");
    }
}
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("{}", self.label);
    }
}
pub struct PrettyButton {
    pub button: Button,
}

impl Draw for PrettyButton {
    fn draw(&self) {
        println!("********");
        self.button.draw();
        println!("********");
    }
}
