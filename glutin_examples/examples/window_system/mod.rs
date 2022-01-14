use core::ffi::c_void;
use core::ptr;

pub mod glutin;

use super::app::application::Application;
use super::app::graphics::Graphics;

pub struct Win32Handle{
    pub hwnd: *mut c_void,
    pub hinstance: *mut c_void,
}

impl Win32Handle{
	pub fn empty()->Self{
		Self{
			hwnd:ptr::null_mut(),
			hinstance:ptr::null_mut()
		}
	}
}

pub enum NativeWindowHandle{
	Win32(Win32Handle)
}

pub trait WindowSystem{

	fn on_draw(&self, app: &Application, graphics:&dyn Graphics){
		app.on_draw(graphics)
	}

	fn on_init(&self, app: &mut Application, handle:NativeWindowHandle){
		app.on_init(handle);
	}

	fn run(self, app:Application)->!;
}