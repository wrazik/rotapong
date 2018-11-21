extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use app::App;
use ball::make_ball;
use commons::*;
use pad::make_pad;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use rand::prelude::*;

mod app;
mod ball;
mod commons;
mod pad;
mod sprite;

fn main() {
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("rota-pong", [WIDTH as u32, HEIGHT as u32])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut rng = thread_rng();
    let mut x_speed: f64 = rng.gen_range(0.5, 1.5);
    if rng.gen_range(0, 10) > 5 {
        x_speed = -x_speed;
    }
    let mut y_speed: f64 = rng.gen_range(0.5, 1.);
    if rng.gen_range(0, 10) > 5 {
        y_speed = -y_speed;
    }

    let mut app = App {
        gl: GlGraphics::new(opengl),
        width: WIDTH,
        height: HEIGHT,
        left_pad: make_pad(Side::LEFT),
        right_pad: make_pad(Side::RIGHT),
        ball: make_ball(x_speed, y_speed),
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(k) = e.press_args() {
            app.key_pressed(&k);
        }
        if let Some(rel_args) = e.release_args() {
            app.release(&rel_args);
        }
        if let Some(r) = e.render_args() {
            app.render(&r);
        }
        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
