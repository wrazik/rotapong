use commons::{WIDTH, HEIGHT, Center};
pub struct Sprite {
    pub center: Center,
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
}
