pub trait Draw {
    fn draw(&self);
}

pub trait Clone {
    fn clone(&self) -> Self;
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>, // dyn Draw 表示动态实现Draw trait
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
        println!("Draw a Button");
    }
}