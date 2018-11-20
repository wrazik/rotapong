use pad::Pad;
use ball::Ball;
use commons::BLACK;
use opengl_graphics::GlGraphics;
use piston::input::{ UpdateArgs, RenderArgs, Button, Key };

pub struct App {
    pub gl: GlGraphics, // OpenGL drawing backend.
    pub left_pad: Pad,  
    pub right_pad: Pad,  
    pub ball: Ball,  
    pub width: f64,
    pub height: f64
}

impl App {
    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        self.gl.draw(args.viewport(), |_c, gl| {
            // Clear the screen.
            clear(BLACK, gl);
        });

        self.left_pad.draw(&mut self.gl, args);
        self.right_pad.draw(&mut self.gl, args);
        self.ball.draw(&mut self.gl, args);
    }

    pub fn score(&mut self) {
        self.ball.reset();
    }

    pub fn update(&mut self, _args: &UpdateArgs) {
        self.left_pad.update();
        self.right_pad.update();
        self.ball.update();

        let (x, y) = self.ball.sprite.get_center_tuple();
        let radius = self.ball.radius;

        let (lx, ly) = self.left_pad.sprite.get_center_tuple();
        let lh = self.left_pad.height;
        let lw = self.left_pad.width;

        let (rx, ry) = self.right_pad.sprite.get_center_tuple();
        let rh = self.right_pad.height;
        // let rw = self.right_pad.width;
        if self.ball.sprite.is_colliding_with(&self.left_pad.sprite)  {
        	self.ball.bounce_x();
        } else if self.ball.sprite.is_colliding_with(&self.right_pad.sprite) {
        	self.ball.bounce_x();
        }
        if (x - radius) < (lw + lx) {
            if (y > ly) && (y < (ly + lh)) {
                self.ball.bounce_x(); 
            }
            else {
                self.score();
            }
        }
        else if (x + radius) > rx {
            if (y > ry) && (y < (ry + rh)) {
                self.ball.bounce_x(); 
            }
            else {
                self.score();
            }
        }


        if y < self.ball.radius {
            self.ball.bounce_y();
        }

        else if y + self.ball.radius > self.height {
            self.ball.bounce_y();
        }

    }

    pub fn reset(&mut self) {
        self.ball.reset();
    }

    pub fn release(&mut self, key: &Button) {
        match key {
            Button::Keyboard(k) => {
                match k {
                    Key::W =>  {
                        self.left_pad.stop();
                    },
                    Key::S => { 
                        self.left_pad.stop();
                    },
                    Key::Up =>  {
                        self.right_pad.stop();
                    },
                    Key::Down => { 
                        self.right_pad.stop();
                    },
                    _ => {} 
                }
            }
            _ => {}
        }
    }

    pub fn key_pressed(&mut self, key: &Button) {
        // Rotate 2 radians per second.
        match key {
            Button::Keyboard(k) => {
                match k {
                    Key::W =>  {
                        self.left_pad.up();
                    },
                    Key::S => { 
                        self.left_pad.down();
                    },
                    Key::Up =>  {
                        self.right_pad.up();
                    },
                    Key::Down => { 
                        self.right_pad.down();
                    },
                    _ => {} 
                }
            }
            _ => {}
        }
    }
}