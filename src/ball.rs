extern crate graphics;

use color::*;
use commons::{Point, HEIGHT, WIDTH};
use game_object::GameObject;
use graphics::*;
use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
use rand::thread_rng;
use rand::Rng;
use sprite::Sprite;

fn make_default_ball_sprite(x_speed: f64, y_speed: f64) -> Sprite {
    let center_of_screen = [(WIDTH as f64 / 2.), (HEIGHT as f64 / 2.)];
    Sprite::new(
        Point {
            x: center_of_screen[0],
            y: center_of_screen[1],
        },
        10.0,
        10.0,
        3.0,
        [x_speed, y_speed],
    )
}

pub struct Ball {
    pub sprite: Sprite,
    pub radius: f64,
    color: Color,
    velocity: [[f64; 2]; 2],
}

impl GameObject for Ball {
    fn draw(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        let center = self.sprite.get_center();

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform.trans(center.x, center.y);

            ellipse(
                self.color.to_rgb(),
                graphics::ellipse::circle(0., 0., self.radius),
                transform,
                gl,
            );
        });
    }

    fn reset(&mut self) {
        let mut rng = thread_rng();
        let mut x_speed: f64 = rng.gen_range(0.5, 1.5);
        let mut y_speed: f64 = rng.gen_range(0.8, 1.3);
        if rng.gen_range(0, 10) > 5 {
            x_speed = -x_speed;
        }
        if rng.gen_range(0, 10) > 5 {
            y_speed = -y_speed;
        }
        self.sprite = make_default_ball_sprite(x_speed, y_speed);
    }

    fn update(&mut self) {
        self.sprite.update();
    }
}

impl Ball {
    pub fn new(x_speed: f64, y_speed: f64, velocity: [[f64; 2]; 2]) -> Ball {
        Ball {
            sprite: make_default_ball_sprite(x_speed, y_speed),
            radius: 10.0,
            color: Color::new(DefinedColors::RED),
            velocity,
        }
    }

    pub fn adjust_y(&mut self, pad_speed: f64) {
        self.sprite.velocity[1] += 0.2 * pad_speed;
    }

    pub fn bounce_x(&mut self) {
        self.sprite.mul_velocity(self.velocity[0]);
    }

    pub fn bounce_y(&mut self) {
        self.sprite.mul_velocity(self.velocity[1]);
    }
}
