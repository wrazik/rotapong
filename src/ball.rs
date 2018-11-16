extern crate graphics;

use rand::thread_rng;
use rand::Rng;
use commons::WHITE;
use sprite::Sprite;
use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
use graphics::*;

pub struct Ball {
    pub sprite: Sprite,
    pub radius: f64,
}

impl Ball {
    pub fn draw(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {

        let (x, y) = (self.sprite.center.x,
                      self.sprite.center.y);

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform.trans(x, y);

            ellipse(
                WHITE,
                graphics::ellipse::circle(0.,0., self.radius), 
                transform, 
                gl
            );
        });
    }
    
    pub fn bounce_x(&mut self) {
       self.sprite.velocity[0] = -self.sprite.velocity[0]*1.1;

    }

    pub fn bounce_y(&mut self) {
       self.sprite.velocity[1] = -self.sprite.velocity[1];
    }

    pub fn update(&mut self) {
        self.sprite.update();
    }
    
    pub fn reset(&mut self) {
        self.sprite.center.x = 400.;
        self.sprite.center.y = 300.;
        let mut rng = thread_rng();
        let mut x_speed: f64 = rng.gen_range(0.5, 1.5);
        if rng.gen_range(0, 10) > 5 {
            x_speed = -x_speed;
        } 
        self.sprite.velocity[0] = x_speed;
        let mut y_speed: f64 = rng.gen_range(0.8, 1.3);
        if rng.gen_range(0, 10) > 5 {
            y_speed = -y_speed;
        } 
        self.sprite.velocity[1] = y_speed;
    }
}