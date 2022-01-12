
pub trait Graphics{

	/// Draw a given rect on desktop
	fn draw_region(&self, x:i32, y:i32, w:u32, h:u32);
}
