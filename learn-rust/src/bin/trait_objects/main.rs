mod state_machine_design_pattern;
mod the_rust_way_state_machine_design_pattern;

fn main() {}

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
    //thr dyn keyword. any type that implements Draw. not homogeneous
    //Box<Button> as well as a Box<TextField>
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw(); // dynamic dispatch
        }
    }
}

pub struct ScreenNot {
    //    pub components: Vec<Box<dyn Clone>>, // cant be used as an object since rust has no casting
}
