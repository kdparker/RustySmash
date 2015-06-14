use piston::event::*;
use opengl_graphics::{ GlGraphics };
use piston::input::keyboard::Key;

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
			player.update(args.dt);
		}
	}

	pub fn new() -> App {
		App {
			players: vec![Player::new(10.0, 10.0, 64, 64)],
		}
	}

	pub fn keypress(&mut self, key: Key) {
		match key {
			Key::W => self.players[0].toggle_direction(Direction::Up, true),
			Key::A => self.players[0].toggle_direction(Direction::Left, true),
			Key::S => self.players[0].toggle_direction(Direction::Down, true),
			Key::D => self.players[0].toggle_direction(Direction::Right, true),
			_      => {}
		}
	}

	pub fn keyrelease(&mut self, key: Key) {
		match key {
			Key::W => self.players[0].toggle_direction(Direction::Up, false),
			Key::A => self.players[0].toggle_direction(Direction::Left, false),
			Key::S => self.players[0].toggle_direction(Direction::Down, false),
			Key::D => self.players[0].toggle_direction(Direction::Right, false),
			_      => {}
		}
	}
}