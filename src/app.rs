use piston_window::*;

use player::*;

pub struct App {
	players: Vec<Player>
}

impl App {
	pub fn render(&mut self, e: PistonWindow) {
		e.draw_2d(|c, gl| {
			clear([1.0; 4], gl);
			for player in self.players.iter() {
				rectangle([0.0, 1.0, 0.0, 1.0], // red
                      player.get_rect(),
                      c.transform, gl);
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