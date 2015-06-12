extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

mod app;
mod player;

fn main() {
	let window = Window::new(
		WindowSettings::new(
			"rusty-smash",
			[1280, 720]
		)
		.exit_on_esc(true)
	);

	let mut app = app::App::new();
	let mut gl = GlGraphics::new(OpenGL::_3_2);
	for e in window.events() {
		if let Some(r) = e.render_args() {
			app.render(&r, &mut gl);
		}

		if let Some(u) = e.update_args() {
			app.update(&u);
		}
	}
}