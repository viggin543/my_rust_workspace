#[derive(Debug)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    pub fn era(&self) -> u32 {
        self.x * self.y
    }

    pub fn print_x(&self , format: &str) {
        println!("{} {}",&self.x, format);
    }
}
