use piston::event::*;
use opengl_graphics::{ GlGraphics };

use player::*;

pub struct App {
	players: Vec<Player>
}

impl App {
	pub fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
		use graphics::*;

		const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 0.0];

		gl.draw(args.viewport(), |c, gl| {
			clear(BLACK, gl);
			for player in self.players.iter() {
				let transform = player.transform(c);
				image(player.get_texture(), transform, gl);
			}
		});
	}

	pub fn update(&mut self, args: &UpdateArgs) {
		for player in self.players.iter_mut() {
			player.change_pos((args.dt, args.dt));
		}
	}

	pub fn new() -> App {
		App {
			players: vec![Player::new(10.0, 10.0, 64, 64)],
		}
	}
}