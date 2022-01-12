use core::ffi::c_void;
use core::ptr;

pub mod glutin;

use super::app::application::Application;

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
	fn run(self, app:Application)->!;
}