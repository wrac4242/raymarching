use crate::types;


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

	pub fn march_pixel(&mut self, pix: types::Point2) -> types::Colour {
		let mut pix3 = types::Point3::new((pix.x/ self.width as f32) - 0.5, (pix.y / self.height as f32) -  0.5, 1.0);
		pix3.normalize();
		pix3 = pix3.mul(255.0/2.0);
		[(pix3.x + 255.0/2.0) as u8, (pix3.y + 255.0/2.0) as u8, 0x00, 0xff]
	}

}
