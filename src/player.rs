use opengl_graphics::{Texture};
use graphics::*;
use std::path::Path;

type Transform = [[f64; 3]; 2];

pub trait Actor {
	fn update(&mut self, f64);
	fn get_pos(&self) -> (i32, i32);
	fn get_height(&self) -> i32;
	fn get_width(&self) -> i32;
	fn transform(&self, c: Context) -> Transform;
	fn get_texture(&self) -> &Texture;
	fn update_velocity(&mut self, i8, i8);
}

pub struct Player {
	rect: (f64, f64, i32, i32),
	sprite: Texture,
	velocity: (i8, i8)
}

impl Actor for Player {
	fn update(&mut self, dt: f64) {
		self.rect = (self.rect.0 + (dt * 10.0 * (self.velocity.0 as f64)), 
					 self.rect.1 + (dt * 10.0 * (self.velocity.1 as f64)), 
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

	fn transform(&self, c: Context) -> Transform {
		c.transform.trans(self.rect.0, self.rect.1)
	}

	fn get_texture(&self) -> &Texture {
		&self.sprite
	}

	fn update_velocity(&mut self, dx: i8, dy: i8) {
		self.velocity.0 += dx;
		self.velocity.1 += dy;
		if self.velocity.0 > 1 { self.velocity.0 = 1; }
		if self.velocity.0 < -1 { self.velocity.0 = -1; }
		if self.velocity.1 > 1 { self.velocity.1 = 1; }
		if self.velocity.1 < -1 { self.velocity.1 = -1; }
	}
}

impl Player {
	pub fn new(x: f64, y: f64, height: i32, width: i32) -> Player {
		Player {
			rect: (x, y, height, width),
			sprite: Some(Texture::from_path(Path::new("./assets/sprites/ness.png"))
												.unwrap())
												.expect("Failed to load ness sprite"),
			velocity: (0, 0)
		}
	}
}