use commons::{WHITE, HEIGHT, WIDTH, Point};
use sprite::{Sprite, make_sprite};
use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;

pub enum Side {
	LEFT,
	RIGHT
}

pub fn make_pad(side: Side) -> Pad {
	Pad {
            sprite: make_default_pad_sprite(side),
            height: 60.,
            width: 20.
        }
}

fn make_default_pad_sprite(side: Side) -> Sprite {
    make_sprite(Point {
        x: match side {
        	Side::LEFT => 20./2.,
        	Side::RIGHT => WIDTH-20./2.
        },
        y: HEIGHT/2. - 30.,
    },
    60.,
    20.,
    3.0,
    [0., 0.])
}

pub struct Pad {
    pub sprite: Sprite,
    pub height: f64,
    pub width: f64
}

impl Pad {

    pub fn draw(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;

        let (x, y) = (0., 0.);
        let center = self.sprite.get_center();

        let pad: [f64; 4] = [center.x, center.y, self.width, self.height];

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform.trans(x, y);

            rectangle(WHITE, pad, transform, gl);
        });
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

    pub fn update(&mut self) {
        self.sprite.update();
    }
}