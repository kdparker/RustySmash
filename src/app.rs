use piston::event::*;
use opengl_graphics::{ GlGraphics, OpenGL };

use actor::Player;

pub struct App {
	gl: GlGraphics,
	players: Vec<Player>
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
			gl: GlGraphics::new(OpenGL::_3_2),
			players: vec![Player::new((10, 10))]
		}
	}
}