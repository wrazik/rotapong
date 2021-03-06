use ball::Ball;
use board::Board;
use color::*;
use commons::*;
use game_object::GameObject;
use opengl_graphics::GlGraphics;
use pad::Pad;
use piston::input::{Button, Key, RenderArgs, UpdateArgs};
use sprite::HorizontalSpritePosition;

pub struct App {
    gl: GlGraphics,
    // OpenGL drawing backend.
    left_pad: Pad,
    right_pad: Pad,
    ball: Ball,
    background_color: Color,
    scoreboard: [u32; 2],
    board: Board,
}

impl App {
    pub fn new(gl: GlGraphics, x_speed: f64, y_speed: f64) -> App {
        let velocity = [[-1.2, 1.2], [1., -1.]];

        App {
            gl,
            left_pad: Pad::new(Side::LEFT),
            right_pad: Pad::new(Side::RIGHT),
            ball: Ball::new(x_speed, y_speed, velocity),
            background_color: Color::new(DefinedColors::CYAN),
            scoreboard: [0, 0],
            board: Board {
                x: HORIZONTAL_MARGIN as f64,
                y: VERTICAL_MARGIN as f64,
                width: WIDTH as f64,
                height: HEIGHT as f64,
            },
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
        self.board.draw(&mut self.gl, args);
    }

    pub fn score(&mut self, side: Side) {
        match side {
            Side::LEFT => self.scoreboard[1] += 1,
            Side::RIGHT => self.scoreboard[0] += 1,
        }
        self.ball.reset();
    }

    pub fn update(&mut self, _args: UpdateArgs) {
        self.left_pad.update();
        self.right_pad.update();
        self.ball.update();

        match self.ball.sprite.is_x_inside_of_play_area() {
            HorizontalSpritePosition::Inside => {}
            HorizontalSpritePosition::Outside(side) => {
                self.score(side);
                return;
            }
        }

        if self.ball.sprite.is_colliding_with(&self.left_pad.sprite) {
            self.ball.bounce_x();
            self.ball.adjust_y(self.left_pad.sprite.get_y_speed());
        } else if self.ball.sprite.is_colliding_with(&self.right_pad.sprite) {
            self.ball.bounce_x();
            self.ball.adjust_y(self.right_pad.sprite.get_y_speed());
        }

        if !self.ball.sprite.is_y_inside_of_play_area() {
            self.ball.bounce_y();
        }
    }

    pub fn release(&mut self, key: &Button) {
        if let Button::Keyboard(k) = key {
            match k {
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
            }
        }
    }

    pub fn key_pressed(&mut self, key: &Button) {
        // Rotate 2 radians per second.
        if let Button::Keyboard(k) = key {
            match k {
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
            }
        }
    }

    pub fn get_title(&self) -> String {
        format!(
            "ROTAPONG! Score: L {} : {} R",
            self.scoreboard[0], self.scoreboard[1]
        )
    }
}
