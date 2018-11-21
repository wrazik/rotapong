use color::*;
use ball::Ball;
use opengl_graphics::GlGraphics;
use pad::Pad;
use piston::input::{Button, Key, RenderArgs, UpdateArgs};
use commons::*;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    left_pad: Pad,
    right_pad: Pad,
    ball: Ball,
    background_color: Color
}

impl App {
    pub fn new(gl: GlGraphics, x_speed: f64, y_speed: f64) -> App {
        App {
            gl: gl,
            left_pad: Pad::new(Side::LEFT),
            right_pad: Pad::new(Side::RIGHT),
            ball: Ball::new(x_speed, y_speed),
            background_color: Color::new(DefinedColors::CYAN)
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let current_color = self.background_color.to_rgb();

        self.gl.draw(args.viewport(), |_c, gl| {
            // Clear the screen.
            clear(current_color, gl);
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
        self.background_color.increment_hue();

        if !self.ball.sprite.is_x_inside_of_play_area() {
            self.score();
            return;
        }

        if self.ball.sprite.is_colliding_with(&self.left_pad.sprite)
            || self.ball.sprite.is_colliding_with(&self.right_pad.sprite)
        {
            self.ball.bounce_x();
        }

        if !self.ball.sprite.is_y_inside_of_play_area() {
            self.ball.bounce_y();
        }
    }

    pub fn reset(&mut self) {
        self.ball.reset();
    }

    pub fn release(&mut self, key: &Button) {
        match key {
            Button::Keyboard(k) => match k {
                Key::W => {
                    self.left_pad.stop();
                }
                Key::S => {
                    self.left_pad.stop();
                }
                Key::Up => {
                    self.right_pad.stop();
                }
                Key::Down => {
                    self.right_pad.stop();
                }
                _ => {}
            },
            _ => {}
        }
    }

    pub fn key_pressed(&mut self, key: &Button) {
        // Rotate 2 radians per second.
        match key {
            Button::Keyboard(k) => match k {
                Key::W => {
                    self.left_pad.up();
                }
                Key::S => {
                    self.left_pad.down();
                }
                Key::Up => {
                    self.right_pad.up();
                }
                Key::Down => {
                    self.right_pad.down();
                }
                _ => {}
            },
            _ => {}
        }
    }
}
