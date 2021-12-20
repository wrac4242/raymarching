use crate::general_types::*;

pub mod primatives;

pub struct ObjectStore {
	objects: Vec<Box<dyn Object>>,
}

impl ObjectStore {
	pub fn new() -> Self {
		ObjectStore {
			objects: Vec::new(),
		}
	}

	pub fn min_distance(&self, position: Point3) -> f32 {
		todo!();
		// objects vector into iterator, runs distance on all, calculates min
	}

	pub fn add_object(&mut self, object: Box<dyn Object>) {
		self.objects.push(object);
	}
}
