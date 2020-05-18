use commons::*;
use game_object::GameObject;
use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;

pub(crate) struct Board {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) width: f64,
    pub(crate) height: f64,
}

impl Board {
    pub fn get_lines(&self) -> [[f64; 4]; 4] {
        let x1 = self.x;
        let y1 = self.y;
        let x2 = self.width + HORIZONTAL_MARGIN as f64;
        let y2 = self.height + VERTICAL_MARGIN as f64;

        [
            [x1, y1, x2, y1], //up
            [x2, y1, x2, y2], //down
            [x1, y1, x1, y2], //left
            [x1, y2, x2, y2], //right
        ]
    }

    fn draw_line(&self, gl: &mut GlGraphics, args: &RenderArgs, coords: [f64; 4]) {
        use graphics::*;

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform.trans(0., 0.);
            line(color::BLACK, 2.0, coords, transform, gl);
        });
    }
}

impl GameObject for Board {
    fn draw(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        let [up, down, left, right] = self.get_lines();
        self.draw_line(gl, args, up);
        self.draw_line(gl, args, down);
        self.draw_line(gl, args, left);
        self.draw_line(gl, args, right);
    }

    fn reset(&mut self) {}
    fn update(&mut self) {}
}
