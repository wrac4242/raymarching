use crate::general_types::*;

pub struct Sphere {
	position: Point3,
	radius: f32,
	colour: Colour,
}

impl Sphere {
	pub fn new(position: Point3, radius: f32, colour: Colour) -> Self {
		Self {
			position: position,
			radius: radius,
			colour: colour,
		}
	}
}

impl Object for Sphere {
	fn distance(&self, point: Point3) -> f32 {
		let distance = (point - self.position).length();
		distance - self.radius
	}
}
