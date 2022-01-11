

pub mod glutin;

use super::app::application::Application;

pub trait WindowSystem{
	fn run(self, app:Application)->!;
}