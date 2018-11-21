extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;
extern crate clap;

use piston::window::AdvancedWindow;
use app::App;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use rand::prelude::*;
use commons::*;
use clap::App as ClapApp;
use clap::Arg;

mod app;
mod ball;
mod commons;
mod pad;
mod sprite;
mod color;
mod game_object;

fn main() {
    let matches = ClapApp::new("ROTAPONG!")
        .arg(Arg::with_name("dynamic-colors").short("d").long("dynamic").help("Make the game more fabulous"))
        .arg(Arg::with_name("fast").short("f").long("fast").help("Make the game twice as fast after each bounce"))
        .get_matches();

    let is_colorful = matches.is_present("dynamic-colors");
    let it_gets_faster = matches.is_present("fast");
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

    let mut app = App::new(GlGraphics::new(opengl), x_speed, y_speed, is_colorful, it_gets_faster);

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
            window.set_title(app.get_title());
        }
    }
}
