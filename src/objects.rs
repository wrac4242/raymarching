use crate::general_types::*;

pub struct ObjectStore {
	pub objects: Vec<Box<dyn Object>>,
}

impl ObjectStore {
	pub fn new() -> Self {
		ObjectStore {
			objects: Vec::new(),
		}
	}
}

pub trait Object {
	fn distance(&self, point: Point3) -> f32;
}
