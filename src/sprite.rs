use commons::{WIDTH, HEIGHT, Point};
pub struct Sprite {
    pub center: Point,
    pub velocity: [f64; 2],
    pub speed: f64
}

impl Sprite {

    pub fn up(&mut self) {
        self.velocity = [0., -self.speed]
    }
    pub fn down(&mut self) {
        self.velocity = [0., self.speed]
    }
    pub fn stop(&mut self) {
        self.velocity = [0., 0.]
    }

    pub fn update(&mut self) {
        self.center.x += self.velocity[0];

        if self.center.x < 0. {
            self.center.x = 0.
        }
        else if self.center.x > WIDTH {
            self.center.x = WIDTH;
        }

        self.center.y += self.velocity[1];
        if self.center.y < 0. {
            self.center.y = 0.
        }

        else if self.center.y > HEIGHT {
            self.center.y = HEIGHT;
        }
    }

    pub fn get_center(&self) -> Point { self.center.clone() }
    pub fn get_center_tuple(&self) -> (f64, f64) { (self.center.x, self.center.y) }
    pub fn set_center(&mut self, x: f64, y: f64) {
        self.center.x = x;
        self.center.y = y;
    } 
}