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
            self.north_west.x = 0.;
            self.south_east.x = self.width;
        }
        else if self.south_east.x > WIDTH {
            self.north_west.x = WIDTH - self.width;
            self.south_east.x = WIDTH;
        }

        self.north_west.y += self.velocity[1];
        self.south_east.y += self.velocity[1];
        
        if self.north_west.y < 0. {
            self.north_west.y = 0.;
            self.south_east.y = self.height;
        }
        else if self.south_east.y > HEIGHT {
            self.north_west.y = HEIGHT - self.height;
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
        )
    }

    pub fn mul_velocity(&mut self, x: f64, y: f64) {
        self.velocity[0] *= x;
        self.velocity[1] *= y;
    }
    pub fn get_polygon(&self) -> [[f64; 2]; 4] {
        [
            [self.north_west.x, self.north_west.y],
            [self.south_east.x, self.north_west.y],
            [self.south_east.x, self.south_east.y],
            [self.north_west.x, self.south_east.y],
        ]
    }

    fn is_vertically_colliding_with(&self, other: &Sprite) -> bool {
        (self.north_west.y < other.south_east.y && self.south_east.y > other.south_east.y) ||
            (self.north_west.y < other.north_west.y && self.south_east.y > other.north_west.y)
    }

    fn is_horizontally_colliding_with(&self, other: &Sprite) -> bool {
        (self.north_west.x < other.south_east.x && self.south_east.x > other.south_east.x) ||
            (self.north_west.x < other.north_west.x && self.south_east.x > other.north_west.x)
    }

    pub fn is_colliding_with(&self, other: &Sprite) -> bool {
        (self.is_vertically_colliding_with(other) && self.is_horizontally_colliding_with(other)) || 
            (other.is_vertically_colliding_with(self) && other.is_horizontally_colliding_with(self))
    }

    pub fn is_x_inside_of_play_area(&self) -> bool {
        (self.north_west.x > 0. && self.south_east.x < WIDTH)
    }

    pub fn is_y_inside_of_play_area(&self) -> bool {
        (self.north_west.y > 0. && self.south_east.y < HEIGHT)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_sprite(nw: [f64; 2], se: [f64; 2]) -> Sprite {
        Sprite {
        north_west: Point {
            x: nw[0],
            y: nw[1]
        },
        south_east: Point {
            x: se[0],
            y: se[1]
        },
        width: 0.0,
        height: 0.0,
        velocity: [0.0, 0.0],
        speed: 0.0
    }
    }

    mod collision_detection {
        use sprite::tests::make_test_sprite;

        #[test]
        fn one_corner() {
            let colliding1 = make_test_sprite([0., 0.], [1., 1.]);
            let colliding2 = make_test_sprite([0.5, 0.5], [2., 2.]);
            assert!(colliding1.is_colliding_with(&colliding2));
            assert!(colliding2.is_colliding_with(&colliding1));
        }

        #[test]
        fn whole_side_y() {
            let colliding1 = make_test_sprite([0., 0.], [3., 3.]);
            let colliding2 = make_test_sprite([0.5, 0.5], [2., 5.]);
            assert!(colliding1.is_colliding_with(&colliding2));
            assert!(colliding2.is_colliding_with(&colliding1));
        }

        #[test]
        fn whole_side_x() {
            let colliding1 = make_test_sprite([0., 0.], [3., 3.]);
            let colliding2 = make_test_sprite([0.5, 0.5], [5., 2.]);
            assert!(colliding1.is_colliding_with(&colliding2));
            assert!(colliding2.is_colliding_with(&colliding1));
        }

        #[test]
        fn inside() {
            let colliding1 = make_test_sprite([0., 0.], [3., 3.]);
            let colliding2 = make_test_sprite([0.5, 0.5], [2., 2.]);
            assert!(colliding1.is_colliding_with(&colliding2));
            assert!(colliding2.is_colliding_with(&colliding1));
        }
    }

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