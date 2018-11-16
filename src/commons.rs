pub const WHITE: [f32; 4] = [0.3, 0.3, 1.0, 1.0];
pub const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
pub const WIDTH: f64 = 640.;
pub const HEIGHT: f64 = 480.;

#[derive(Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64
}

pub struct Size {
	pub width: f64,
	pub height: f64
}
