extern crate graphics;

use rand::thread_rng;
use rand::Rng;
use commons::{WHITE, WIDTH, HEIGHT, Point};
use sprite::{Sprite, make_sprite};
use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
use graphics::*;

pub fn make_ball(x_speed: f64, y_speed: f64) -> Ball {
    Ball {
            sprite: make_default_ball_sprite(x_speed, y_speed),
            radius: 10.0
        }
}

fn make_default_ball_sprite(x_speed: f64, y_speed: f64) -> Sprite {
    make_sprite(Point {
        x: WIDTH/2.,
        y: HEIGHT/2.,
    },
    10.0,
    10.0,
    3.0,
    [x_speed, y_speed])
}

pub struct Ball {
    pub sprite: Sprite,
    pub radius: f64,
}

impl Ball {
    pub fn draw(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {

        let center = self.sprite.get_center();

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform.trans(center.x, center.y);

            ellipse(
                WHITE,
                graphics::ellipse::circle(0.,0., self.radius), 
                transform, 
                gl
            );
        });
    }
    
    pub fn bounce_x(&mut self) {
        self.sprite.mul_velocity(-1.0, 1.0);
    }

    pub fn bounce_y(&mut self) {
        self.sprite.mul_velocity(1.0, -1.1);
    }

    pub fn update(&mut self) {
        self.sprite.update();
    }
    
    pub fn reset(&mut self) {
        // self.sprite.set_center(400., 300.);
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
        // self.sprite.velocity[0] = x_speed;
        // self.sprite.velocity[1] = y_speed;
    }
}