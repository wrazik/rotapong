use commons::WHITE;
use sprite::Sprite;
use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;

pub struct Pad {
    pub sprite: Sprite,
    pub height: f64,
    pub width: f64
}

impl Pad {

    pub fn draw(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;

        let (x, y) = (0., 0.);

        let pad: [f64; 4] = [self.sprite.center.x, self.sprite.center.y, self.width, self.height];

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