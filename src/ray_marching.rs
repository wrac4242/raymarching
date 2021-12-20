use crate::general_types::*;
use crate::objects;

pub struct Renderer {
	width: u32,
	height: u32,
	object_list: objects::ObjectStore
}

impl Renderer {
	pub fn new(width: u32, height: u32) -> Self {
		Self {
			width: width,
			height: height,
			object_list: Renderer::object_init(),
		}
	}

	fn object_init() -> objects::ObjectStore {
		let mut obj_store = objects::ObjectStore::new();
		obj_store.add_object(Box::new(objects::primitives::Sphere::new(
			Point3::new(0.0, 0.0, 2.0),
			0.5,
			colour_types::new(255, 0, 0, 255),
		)));
		obj_store.add_object(Box::new(objects::primitives::Sphere::new(
			Point3::new(0.5, 0.5, 2.0),
			0.5,
			colour_types::new(0, 255, 0, 255),
		)));
		obj_store.add_object(Box::new(objects::primitives::Sphere::new(
			Point3::new(-0.5, -0.5, 2.0),
			0.5,
			colour_types::new(0, 0, 255, 255),
		)));
		obj_store
	}

	const MAXDISTANCE: f32 = 100.0;
	const MINDISTANCE: f32 = 0.01;
	const BACKGROUND_COLOUR: colour_types::Colour = [0, 0, 0, 255];
	const MAX_STEPS: u32 = 100;


	pub fn march_pixel(&mut self, pix: Point2) -> colour_types::Colour {
		// calculates the pixels xy position on the screen to be between -1 and 1
		let mut pos = Point3::new(((2.0 * pix.x)/ self.width as f32) - 1.0, ((2.0 * pix.y) / self.height as f32) -  1.0, 1.0); 
		let mut normalized_pos = pos;
		normalized_pos.normalize();
		
		let mut count: u32 = 0;
		loop {
			count += 1;
			let (min_distance, min_colour) = self.object_list.min_distance(pos);
			if min_distance < Renderer::MINDISTANCE {
				return min_colour;
			}
			if count > Renderer::MAX_STEPS || min_distance > Renderer::MAXDISTANCE {
				return Renderer::BACKGROUND_COLOUR;
			}
			pos += normalized_pos * min_distance;
		}
	}

}
