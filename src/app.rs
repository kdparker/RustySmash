use piston::event::*;
use opengl_graphics::{ GlGraphics, OpenGL };

use player::*;

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

	pub fn update(&mut self, args: &UpdateArgs) {
		for player in self.players.iter_mut() {
			player.change_pos((args.dt, args.dt));
		}
	}

	pub fn new() -> App {
		App {
			gl: GlGraphics::new(OpenGL::_3_2),
			players: vec![Player::new(10.0, 10.0, 64, 64)]
		}
	}
}