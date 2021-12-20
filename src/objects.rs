use crate::general_types::*;

pub mod primitives;

pub struct ObjectStore {
	objects: Vec<Box<dyn Object>>,
}

impl ObjectStore {
	pub fn new() -> Self {
		ObjectStore {
			objects: Vec::new(),
		}
	}

	pub fn min_distance(&self, position: Point3) -> (f32, colour_types::Colour) {
		// objects vector into iterator, runs distance on all, calculates min, returns min and min colour
		self.objects.iter().map(|obj| obj.distance(position)).min_by(|a, b| a.0.partial_cmp(&b.0).unwrap()).unwrap()
	}

	pub fn add_object(&mut self, object: Box<dyn Object>) {
		self.objects.push(object);
	}
}

