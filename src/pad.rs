use color::*;
use commons::{Point, Side, HEIGHT, WIDTH};
use game_object::GameObject;
use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
use sprite::Sprite;

fn make_default_pad_sprite(side: Side) -> Sprite {
    let pad_position = [
        match side {
            Side::LEFT => 20. / 2.,
            Side::RIGHT => WIDTH as f64 - 20. / 2.,
        },
        (HEIGHT as f64 / 2. - 30.),
    ];
    Sprite::new(
        Point {
            x: pad_position[0],
            y: pad_position[1],
        },
        20.,
        120.,
        3.0,
        [0., 0.],
    )
}

pub struct Pad {
    pub sprite: Sprite,
    pub height: f64,
    pub width: f64,
    color: Color,
}

impl GameObject for Pad {
    fn draw(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform.trans(0., 0.);

            polygon(
                self.color.to_rgb(),
                &self.sprite.get_polygon(),
                transform,
                gl,
            );
        });
    }

    fn update(&mut self) {
        self.sprite.update();
    }

    fn reset(&mut self) {}
}

impl Pad {
    pub fn new(side: Side) -> Pad {
        Pad {
            sprite: make_default_pad_sprite(side),
            height: 60.,
            width: 20.,
            color: Color::new(DefinedColors::RED),
        }
    }

    pub fn up(&mut self) {
        self.sprite.up();
    }

    pub fn down(&mut self) {
        self.sprite.down();
    }

    pub fn stop(&mut self) {
        self.sprite.stop();
    }
}
