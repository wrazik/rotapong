pub const WIDTH: u32 = 640;
pub const HEIGHT: u32 = 480;
pub const VERTICAL_MARGIN: u32 = 100;
pub const HORIZONTAL_MARGIN: u32 = 150;

pub enum Side {
    LEFT,
    RIGHT,
}

#[derive(Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
