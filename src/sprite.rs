use commons::{WIDTH, HEIGHT, Point};

pub fn make_sprite(center: Point, width: f64, height: f64, speed: f64, velocity: [f64; 2]) -> Sprite {
    Sprite {
        north_west: Point {
            x: center.x - width/2.0,
            y: center.y - height/2.0
        },
        south_east: Point {
            x: center.x + width/2.0,
            y: center.y + height/2.0
        },
        width: width,
        height: height,
        velocity: velocity,
        speed: speed
    }
}

pub struct Sprite {
    north_west: Point,
    south_east: Point,
    width: f64,
    height: f64,
    velocity: [f64; 2],
    speed: f64
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
        self.north_west.x += self.velocity[0];
        self.south_east.x += self.velocity[0];

        if self.north_west.x < 0. {
            self.north_west.x = 0.
        }
        else if self.south_east.x > WIDTH {
            self.south_east.x = WIDTH;
        }

        self.north_west.y += self.velocity[1];
        self.south_east.y += self.velocity[1];
        if self.north_west.y < 0. {
            self.north_west.y = 0.
        }

        else if self.south_east.y > HEIGHT {
            self.south_east.y = HEIGHT;
        }
    }

    pub fn get_center(&self) -> Point {
        Point {
            x: (self.north_west.x + self.south_east.x) / 2.0,
            y: (self.north_west.y + self.south_east.y) / 2.0,
        }}
    pub fn get_center_tuple(&self) -> (f64, f64) { (
            (self.north_west.x + self.south_east.x) / 2.0,
            (self.north_west.y + self.south_east.y) / 2.0
        ) }
    pub fn set_center(&mut self, x: f64, y: f64) {
        self.north_west = Point {
            x: x - self.width/2.0,
            y: y - self.height/2.0
        };
        self.south_east = Point {
            x: x + self.width/2.0,
            y: y + self.height/2.0
        };
    }
    pub fn mul_velocity(&mut self, x: f64, y: f64) {
        self.velocity[0] *= x;
        self.velocity[1] *= y;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_center_and_get_center_tuple() {
        let sprite = make_sprite(Point { 
                    x: 1.0, 
                    y: 2.0,
                }, 4.0, 2.0, 3.0, [0., 0.]);
        assert_eq!(sprite.get_center().x, 1.0);
        assert_eq!(sprite.get_center().y, 2.0);
        assert_eq!(sprite.get_center_tuple(), (1.0, 2.0));
    }
}