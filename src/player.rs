pub trait Actor {
	fn change_pos(&mut self, (f64, f64));
	fn get_pos(&self) -> (i32, i32);
}

pub struct Player {
	pos: (f64, f64)
}

impl Actor for Player {
	fn change_pos(&mut self, dp: (f64, f64)) {
		self.pos = (self.pos.0 + dp.0, self.pos.1 + dp.1);
		println!("({},{})", self.pos.0, self.pos.1);
	}

	fn get_pos(&self) -> (i32, i32) {
		(self.pos.0 as i32, self.pos.1 as i32)
	}
}

impl Player {
	pub fn new(pos: (f64, f64)) -> Player {
		Player {
			pos: pos
		}
	}
}