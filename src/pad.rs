use game_object::GameObject;
use commons::{Point, Side, HEIGHT, WIDTH};
use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
use sprite::{make_sprite, Sprite};
use color::*;

fn make_default_pad_sprite(side: Side) -> Sprite {
    make_sprite(
        Point {
            x: match side {
                Side::LEFT => 20. / 2.,
                Side::RIGHT => WIDTH - 20. / 2.,
            },
            y: HEIGHT / 2. - 30.,
        },
        20.,
        120.,
        8.0,
        [0., 0.],
    )
}

pub struct Pad {
    pub sprite: Sprite,
    pub height: f64,
    pub width: f64,
    color: Color,
    update_hook: Box<fn(&mut Color)>
}

impl GameObject for Pad {
    fn draw(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;

        let (x, y) = (0., 0.);

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform.trans(x, y);

            polygon(self.color.to_rgb(), &self.sprite.get_polygon(), transform, gl);
        });
    }

    fn update(&mut self) {
        (self.update_hook)(&mut self.color);
        self.sprite.update();
    }

    fn reset(&mut self) {}
}

impl Pad {
    pub fn new(side: Side, update_hook: Box<fn(&mut Color)>) -> Pad {
        Pad {
            sprite: make_default_pad_sprite(side),
            height: 60.,
            width: 20.,
            color: Color::new(DefinedColors::RED),
            update_hook: update_hook
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
