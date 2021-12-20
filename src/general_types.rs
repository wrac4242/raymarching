use std::ops::{Sub, Add, Mul, Div};

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
		self.x /= length;
		self.y /= length;
		self.z /= length;
	}

	pub fn length(&self) -> f32 {
		(self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
	}

}

impl Sub for Point3 {
	type Output = Point3;
	fn sub(self, other: Point3) -> Point3 {
		Point3 {
			x: self.x - other.x,
			y: self.y - other.y,
			z: self.z - other.z
		}
	}
}

impl Add for Point3 {
	type Output = Point3;
	fn add(self, other: Point3) -> Point3 {
		Point3 {
			x: self.x + other.x,
			y: self.y + other.y,
			z: self.z + other.z
		}
	}
}

impl Mul for Point3 {
	type Output = Point3;
	fn mul(self, other: Point3) -> Point3 {
		Point3 {
			x: self.x * other.x,
			y: self.y * other.y,
			z: self.z * other.z
		}
	}
}

impl Div for Point3 {
	type Output = Point3;
	fn div(self, other: Point3) -> Point3 {
		Point3 {
			x: self.x / other.x,
			y: self.y / other.y,
			z: self.z / other.z
		}
	}
}


pub type Colour = [u8; 4];

pub mod colour_types {
	use crate::general_types::Colour;
	pub fn new(r: u8, g: u8, b: u8, a: u8) -> Colour {
		[r, g, b, a]
	}
}


pub trait Object {
	fn distance(&self, point: Point3) -> f32;
}
