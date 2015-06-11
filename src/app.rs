use piston::event::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

pub struct App {
	gl: GlGraphics
}

impl App {
	pub fn render(&mut self, args: &RenderArgs) {
		use graphics::*;

		const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 0.0];

		self.gl.draw(args.viewport(), |c, gl| {
			clear(BLACK, gl);
		});
	}

	pub fn new() -> App {
		App {
			gl: GlGraphics::new(OpenGL::_3_2)
		}
	}
}