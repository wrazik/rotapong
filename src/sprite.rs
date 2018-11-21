use commons::{Point, HEIGHT, WIDTH, Side};

pub enum HorizontalSpritePosition {
    Inside,
    Outside(Side)
}

pub struct Sprite {
    upper_left: Point,
    lower_right: Point,
    width: f64,
    height: f64,
    velocity: [f64; 2],
    speed: f64,
}

impl Sprite {
    pub fn new(
        center: Point,
        width: f64,
        height: f64,
        speed: f64,
        velocity: [f64; 2]) -> Sprite {
            Sprite {
        upper_left: Point {
            x: center.x - width / 2.0,
            y: center.y - height / 2.0,
        },
        lower_right: Point {
            x: center.x + width / 2.0,
            y: center.y + height / 2.0,
        },
        width: width,
        height: height,
        velocity: velocity,
        speed: speed,
    }
    }
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
        self.upper_left.x += self.velocity[0];
        self.lower_right.x += self.velocity[0];

        if self.upper_left.x < 0. {
            self.upper_left.x = 0.;
            self.lower_right.x = self.width;
        } else if self.lower_right.x > WIDTH {
            self.upper_left.x = WIDTH - self.width;
            self.lower_right.x = WIDTH;
        }

        self.upper_left.y += self.velocity[1];
        self.lower_right.y += self.velocity[1];

        if self.upper_left.y < 0. {
            self.upper_left.y = 0.;
            self.lower_right.y = self.height;
        } else if self.lower_right.y > HEIGHT {
            self.upper_left.y = HEIGHT - self.height;
            self.lower_right.y = HEIGHT;
        }
    }

    pub fn get_center(&self) -> Point {
        Point {
            x: (self.upper_left.x + self.lower_right.x) / 2.0,
            y: (self.upper_left.y + self.lower_right.y) / 2.0,
        }
    }
    pub fn get_center_tuple(&self) -> (f64, f64) {
        (
            (self.upper_left.x + self.lower_right.x) / 2.0,
            (self.upper_left.y + self.lower_right.y) / 2.0,
        )
    }

    pub fn mul_velocity(&mut self, multiplier: [f64; 2]) {
        self.velocity[0] *= multiplier[0];
        self.velocity[1] *= multiplier[1];
    }
    pub fn get_polygon(&self) -> [[f64; 2]; 4] {
        [
            [self.upper_left.x, self.upper_left.y],
            [self.lower_right.x, self.upper_left.y],
            [self.lower_right.x, self.lower_right.y],
            [self.upper_left.x, self.lower_right.y],
        ]
    }

    fn is_vertically_colliding_with(&self, other: &Sprite) -> bool {
        (self.upper_left.y < other.lower_right.y && self.lower_right.y > other.lower_right.y)
            || (self.upper_left.y < other.upper_left.y && self.lower_right.y > other.upper_left.y)
    }

    fn is_horizontally_colliding_with(&self, other: &Sprite) -> bool {
        (self.upper_left.x < other.lower_right.x && self.lower_right.x > other.lower_right.x)
            || (self.upper_left.x < other.upper_left.x && self.lower_right.x > other.upper_left.x)
    }

    pub fn is_colliding_with(&self, other: &Sprite) -> bool {
        (self.is_vertically_colliding_with(other) && self.is_horizontally_colliding_with(other))
            || (other.is_vertically_colliding_with(self)
                && other.is_horizontally_colliding_with(self))
    }

    pub fn is_x_inside_of_play_area(&self) -> HorizontalSpritePosition {
        if self.upper_left.x <= 0. { return HorizontalSpritePosition::Outside(Side::LEFT); }
        else if self.lower_right.x >= WIDTH { return HorizontalSpritePosition::Outside(Side::RIGHT); }
        else { HorizontalSpritePosition::Inside }
    }

    pub fn is_y_inside_of_play_area(&self) -> bool {
        (self.upper_left.y > 0. && self.lower_right.y < HEIGHT)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_sprite(nw: [f64; 2], se: [f64; 2]) -> Sprite {
        Sprite {
            upper_left: Point { x: nw[0], y: nw[1] },
            lower_right: Point { x: se[0], y: se[1] },
            width: 0.0,
            height: 0.0,
            velocity: [0.0, 0.0],
            speed: 0.0,
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
        let sprite = make_sprite(Point { x: 1.0, y: 2.0 }, 4.0, 2.0, 3.0, [0., 0.]);
        assert_eq!(sprite.get_center().x, 1.0);
        assert_eq!(sprite.get_center().y, 2.0);
        assert_eq!(sprite.get_center_tuple(), (1.0, 2.0));
    }
}
