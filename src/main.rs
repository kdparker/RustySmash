extern crate piston_window;

use piston_window::*;

mod app;
mod player;

fn main() {
	let opengl = OpenGL::_3_2;

	let window: PistonWindow = 
		WindowSettings::new(
			"Rusty Smash",
			[1280, 720]
		).exit_on_esc(true).opengl(opengl).into();

	let mut app = app::App::new();
	for e in window {
		if let Some(Button::Keyboard(key)) = e.press_args() {
			app.keypress(key)
		}

		if let Some(Button::Keyboard(key)) = e.release_args() {
			app.keyrelease(key)
		}

		if let Some(u) = e.update_args() {
			app.update(&u);
		}

		app.render(e);
	}
}