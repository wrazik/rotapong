pub const WIDTH: f64 = 640.;
pub const HEIGHT: f64 = 480.;

pub enum Side {
    LEFT,
    RIGHT,
}

#[derive(Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
