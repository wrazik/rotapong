extern crate graphics;

use color::*;
use commons::{Point, HEIGHT, WIDTH};
use graphics::*;
use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
use rand::thread_rng;
use rand::Rng;
use sprite::{make_sprite, Sprite};

fn make_default_ball_sprite(x_speed: f64, y_speed: f64) -> Sprite {
    make_sprite(
        Point {
            x: WIDTH / 2.,
            y: HEIGHT / 2.,
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
    color: Color
}

impl Ball {
    pub fn new(x_speed: f64, y_speed: f64) -> Ball {
        Ball {
            sprite: make_default_ball_sprite(x_speed, y_speed),
            radius: 10.0,
            color: Color::new(DefinedColors::RED)
        }
    }

    pub fn draw(&self, gl: &mut GlGraphics, args: &RenderArgs) {
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

    pub fn bounce_x(&mut self) {
        self.sprite.mul_velocity(-2.0, 2.0);
    }

    pub fn bounce_y(&mut self) {
        self.sprite.mul_velocity(1.0, -1.0);
    }

    pub fn update(&mut self) {
        self.sprite.update();
        self.color.increment_hue();
    }

    pub fn reset(&mut self) {
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
}
