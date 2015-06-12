use opengl_graphics::{Texture};
use graphics::*;
use std::path::Path;

type Transform = [[f64; 3]; 2];

pub trait Actor {
	fn change_pos(&mut self, (f64, f64));
	fn get_pos(&self) -> (i32, i32);
	fn get_height(&self) -> i32;
	fn get_width(&self) -> i32;
	fn transform(&self, c: Context) -> Transform;
	fn get_texture(&self) -> &Texture;
}

pub struct Player {
	rect: (f64, f64, i32, i32),
	sprite: Texture
}

impl Actor for Player {
	fn change_pos(&mut self, dp: (f64, f64)) {
		self.rect = (self.rect.0 + (dp.0 * 10.0), self.rect.1 + (dp.1 * 10.0), self.rect.2, self.rect.3);
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

	fn transform(&self, c: Context) -> Transform {
		c.transform.trans(self.rect.0, self.rect.1)
	}

	fn get_texture(&self) -> &Texture {
		&self.sprite
	}
}

impl Player {
	pub fn new(x: f64, y: f64, height: i32, width: i32) -> Player {
		Player {
			rect: (x, y, height, width),
			sprite: Some(Texture::from_path(Path::new("./assets/sprites/ness.png")).unwrap()).expect("Failed to load ness sprite")
		}
	}
}