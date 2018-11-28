use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;

pub trait GameObject {
    fn draw(&self, gl: &mut GlGraphics, args: &RenderArgs);
    fn reset(&mut self);
    fn update(&mut self);
}
