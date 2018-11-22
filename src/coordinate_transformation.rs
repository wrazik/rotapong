use commons::{HORIZONTAL_MARGIN, VERTICAL_MARGIN};

pub fn transform_x(x: f64) -> f64 {
    x + HORIZONTAL_MARGIN as f64
}
pub fn transform_y(y: f64) -> f64 {
    y + VERTICAL_MARGIN as f64
}
