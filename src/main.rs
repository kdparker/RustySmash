extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

pub struct App {
	gl: GlGraphics
}

impl App {
	fn render(&mut self, args: &RenderArgs) {
		use graphics::*;

		const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 0.0];

		self.gl.draw(args.viewport(), |c, gl| {
			clear(BLACK, gl);
		});
	}
}

fn main() {
	let opengl = OpenGL::_3_2;

	let window = Window::new(
		WindowSettings::new(
			"rusty-smash",
			[1280, 720]
		)
		.exit_on_esc(true)
	);

	let mut app = App {
		gl: GlGraphics::new(opengl)
	};

	for e in window.events() {
		if let Some(r) = e.render_args() {
			app.render(&r);
		}
	}
}