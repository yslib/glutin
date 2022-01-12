use std::path::PathBuf;

use glutin::dpi::{LogicalPosition, LogicalSize, PhysicalSize, Position};
use glutin::event::{Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::monitor::MonitorHandle;
use glutin::window::{Fullscreen, WindowBuilder};
use glutin::ContextBuilder;

use crate::misc::KeyCode;
use crate::misc::shortcutkey::{get_lut, ShortcutTrigger, ShortcutTriggerBuilder, State};
use crate::window_system::{WindowSystem, NativeWindowHandle, Win32Handle};

use super::graphics::Graphics;

pub struct ApplicationBuilder<W:WindowSystem>{
	app_name: String,
	config_file_path:PathBuf,
	window_system:Option<W>
}

impl<W> ApplicationBuilder<W> where W: WindowSystem{

	pub fn new()->Self{
		ApplicationBuilder{
			app_name:"".to_owned(),
			config_file_path:PathBuf::from("".to_owned()),
			window_system:None
		}
	}

	pub fn with_name(mut self, name: String)->Self{
		self.app_name = name;
		self
	}

	pub fn with_config_file_path<T:AsRef<PathBuf>>(mut self, path: T)->Self{
		self.config_file_path = path.as_ref().to_owned();
		self
	}

	pub fn with_window_system(mut self, window_system:W)->Self{
		self.window_system = Some(window_system);
		self
	}

	pub fn exec(self)->!{
		let lut = get_lut();
		let mut key_trigger = ShortcutTriggerBuilder::<_>::new(lut)
			.with_shortcut("Ctrl+Alt+Key2".to_owned(), Box::new(|| println!("set unvisible")))
			.with_shortcut("Ctrl+Alt+Key1".to_owned(), Box::new(|| ()))
			.build()
			.unwrap();

		let app = Application{key_trigger:std::cell::RefCell::new(key_trigger), name:self.app_name.clone()};
		self.window_system.unwrap().run(app)
	}
}


pub struct Application{
	key_trigger:std::cell::RefCell<ShortcutTrigger<String, KeyCode>>,
	name:String,
}

pub struct Image{

}

pub trait CaptureDeviceContext{
	fn capture(&self)->Image;
}

impl Application{
	pub fn get_app_name(&self)->String{
		self.name.clone()
	}

	pub fn on_init(&mut self, native_window:NativeWindowHandle){

	}

	pub fn on_keyboard_event(&self, input: KeyCode){
		self.key_trigger.borrow_mut().trigger(input);
	}

	pub fn on_update(&self){

	}

	pub fn on_draw(&self, graphics: &dyn Graphics){
		graphics.draw_region(100, 100, 100, 100);
	}
}

