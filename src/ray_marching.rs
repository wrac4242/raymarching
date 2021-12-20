use crate::general_types;
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
		obj_store.add_object(Box::new(objects::primatives::Sphere::new(
			general_types::Point3::new(0.0, 0.0, -1.0),
			0.5,
			general_types::colour_types::new(255, 0, 0, 255)
		)));
		obj_store
	}

	pub fn march_pixel(&mut self, pix: general_types::Point2) -> general_types::Colour {
		// calculates the pixels xy position on the screen to be between -1 and 1
		let mut pos = general_types::Point3::new(((2.0 * pix.x)/ self.width as f32) - 1.0, ((2.0 * pix.y) / self.height as f32) -  1.0, 1.0); 
		let mut normalized_pos = pos;
		normalized_pos.normalize();
		
		// runs min distance on object store
		let min_distance = self.object_list.min_distance(pos);
		[(normalized_pos.x + 255.0/2.0) as u8, (normalized_pos.y + 255.0/2.0) as u8, 0x00, 0xff]
	}

}
