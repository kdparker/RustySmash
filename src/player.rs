

pub trait Actor {
	fn change_pos(&mut self, (f64, f64));
	fn get_pos(&self) -> (i32, i32);
	fn get_height(&self) -> i32;
	fn get_width(&self) -> i32;
	fn get_rect(&self) -> (f64, f64, i32, i32);
}

pub struct Player {
	rect: (f64, f64, i32, i32)
}

impl Actor for Player {
	fn change_pos(&mut self, dp: (f64, f64)) {
		self.rect = (self.rect.0 + dp.0, self.rect.1 + dp.1, self.rect.2, self.rect.3);
		println!("({},{})", self.rect.0, self.rect.1);
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

	fn get_rect(&self) -> (f64, f64, i32, i32) {
		self.rect
	}
}

impl Player {
	pub fn new(x: f64, y: f64, height: i32, width: i32) -> Player {
		Player {
			rect: (x, y, height, width)
		}
	}
}