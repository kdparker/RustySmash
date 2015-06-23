use piston_window::*;

type Transform = [[f64; 3]; 2];

pub enum Direction {
	Left,
	Right,
	Up,
	Down
}

fn direction_to_int(dir: Direction) -> usize {
	match dir {
		Direction::Left  => 0,
		Direction::Right => 1,
		Direction::Up    => 2,
		Direction::Down  => 3
	}
}

pub trait Actor {
	fn update(&mut self, f64);
	fn get_pos(&self) -> (i32, i32);
	fn get_height(&self) -> i32;
	fn get_width(&self) -> i32;
	fn toggle_direction(&mut self, Direction, bool);
	fn get_rect(&self) -> [f64; 4];
}

pub struct Player {
	rect: (f64, f64, i32, i32),
	direction: [bool; 4],
	velocity: [f64; 2]
}

impl Actor for Player {
	fn update(&mut self, dt: f64) {
		self.update_velocity();
		self.rect = (self.rect.0 + (dt * 10.0 * (self.velocity[0] as f64)), 
					 self.rect.1 + (dt * 10.0 * (self.velocity[1] as f64)), 
					 self.rect.2, 
					 self.rect.3);
		// println!("Player pos: ({},{})", self.velocity.0, self.velocity.1);
	}

	fn get_pos(&self) -> (i32, i32) {
		(self.rect.0 as i32, self.rect.1 as i32)
	}

	fn get_height(&self) -> i32 {
		self.rect.2
	}

	fn get_width(&self) -> i32 {
		self.rect.3
	}

	fn get_rect(&self) -> [f64; 4] {
		[self.rect.0, self.rect.1, self.rect.2 as f64, self.rect.3 as f64]
	}

	fn toggle_direction(&mut self, dir: Direction, on_off: bool) {
		self.direction[direction_to_int(dir)] = on_off;
	}
}

impl Player {
	fn update_velocity(&mut self) {
		let mut velocity = [0.0, 0.0];
		if self.direction[direction_to_int(Direction::Left)] { velocity[0] -= 1.0; }
		if self.direction[direction_to_int(Direction::Right)] { velocity[0] += 1.0; }
		if self.direction[direction_to_int(Direction::Up)] { velocity[1] -= 1.0; }
		if self.direction[direction_to_int(Direction::Down)] { velocity[1] += 1.0; }

		self.velocity = velocity;
	}

	pub fn new(x: f64, y: f64, height: i32, width: i32) -> Player {
		Player {
			rect: (x, y, height, width),
			direction: [false, false, false, false],
			velocity: [0.0, 0.0]
		}
	}
}