use crate::general_types;
use crate::objects;

pub struct Renderer {
	width: u32,
	height: u32,
}

impl Renderer {
	pub fn new(width: u32, height: u32) -> Self {
		Self {
			width: width,
			height: height,
		}
	}

	pub fn march_pixel(&mut self, pix: general_types::Point2, object_list: &objects::ObjectStore) -> general_types::Colour {
		// calculates the pixels xy position on the screen to be between -1 and 1
		let mut pos = general_types::Point3::new(((2.0 * pix.x)/ self.width as f32) - 1.0, ((2.0 * pix.y) / self.height as f32) -  1.0, 1.0); 
		let mut normalized_pos = pos;
		normalized_pos.normalize();
		normalized_pos = normalized_pos.mul(255.0/2.0);
		[(normalized_pos.x + 255.0/2.0) as u8, (normalized_pos.y + 255.0/2.0) as u8, 0x00, 0xff]
	}

}
