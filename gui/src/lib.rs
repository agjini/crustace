pub trait Draw {
    fn draw(&self) {
        self.default_draw();
    }
    fn default_draw(&self) {
        println!("default_draw");
    }
}

#[derive(Default)]
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }

    pub fn add<T: Draw + 'static>(&mut self, component: T) {
        self.components.push(Box::new(component));
    }
}

impl Screen {
    pub fn new() -> ScreenBuilder {
        ScreenBuilder::default()
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

#[derive(Default)]
pub struct ScreenBuilder {
    pub components: Vec<Box<dyn Draw>>,
}

impl ScreenBuilder {
    pub fn add<T: Draw + 'static>(mut self, component: T) -> Self {
        self.components.push(Box::new(component));
        self
    }

    pub fn build(self) -> Screen {
        Screen {
            components: self.components,
        }
    }
}
