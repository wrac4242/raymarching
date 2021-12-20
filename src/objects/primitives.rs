use crate::general_types::*;

pub struct Sphere {
	position: Point3,
	radius: f32,
	colour: colour_types::Colour,
}

impl Sphere {
	pub fn new(position: Point3, radius: f32, colour: colour_types::Colour) -> Self {
		Self {
			position: position,
			radius: radius,
			colour: colour,
		}
	}
}

impl Object for Sphere {
	fn distance(&self, point: Point3) -> (f32, colour_types::Colour) {
		let distance = (point - self.position).length();
		(distance - self.radius, self.colour)
	}
}

pub struct Plane {
	normal: Point3,
	colour: colour_types::Colour,
	offset: Point3,
	normal_len: f32,
}

impl Plane {
	pub fn new(normal: Point3, colour: colour_types::Colour, offset: Point3) -> Self {
		Self {
			normal: normal,
			colour: colour,
			offset: offset,
			normal_len: normal.length(),
		}
	}
}

impl Object for Plane {
	fn distance(&self, point: Point3) -> (f32, colour_types::Colour) {
		let distance = self.normal.dot(self.offset - point) / self.normal_len;
		(distance, self.colour)
	}
}
