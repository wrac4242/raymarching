#[derive(Copy, Clone)]
pub struct Point3 {
	pub x: f32,
	pub y: f32,
	pub z: f32
}

#[derive(Copy, Clone)]
pub struct Point2 {
	pub x: f32,
	pub y: f32
}

impl Point2 {
	pub fn new(x: f32, y: f32) -> Point2 {
		Point2 {
			x: x,
			y: y
		}
	}
}

impl Point3 {
	pub fn new (x: f32, y: f32, z: f32) -> Point3 {
		Point3 {
			x: x,
			y: y,
			z: z
		}
	}
	pub fn normalize(&mut self) {
		let length = self.length();
		self.x = self.x / length;
		self.y = self.y / length;
		self.z = self.z / length;
	}

	fn length(&self) -> f32 {
		(self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
	}

	pub fn mul(&self, scalar: f32) -> Point3 {
		Point3 {
			x: self.x * scalar,
			y: self.y * scalar,
			z: self.z * scalar
		}
	}
}

pub type Colour = [u8; 4];
