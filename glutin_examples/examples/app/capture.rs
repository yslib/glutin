
use super::image::Image;
pub trait CaptureDeviceContext{
	fn capture(&self)->Image;
}