struct Actor {
	pos: (i32, i32)
}

impl Actor {
	fn new(pos: (i32, i32)) -> Actor {
		Actor {
			pos: pos
		}
	}
}

pub struct Player {
	actor: Actor
}

impl Player {
	pub fn new(pos: (i32, i32)) -> Player {
		Player {
			actor: Actor::new(pos)
		}
	}
}