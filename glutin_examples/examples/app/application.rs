use glutin::dpi::{LogicalPosition, LogicalSize, PhysicalSize, Position};
use glutin::event::{Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::monitor::MonitorHandle;
use glutin::window::{Fullscreen, WindowBuilder};
use glutin::ContextBuilder;

use crate::misc::KeyCode;
use crate::misc::shortcutkey::{get_lut, ShortcutTrigger, ShortcutTriggerBuilder, State};


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
	pub fn new()->Self{
		let lut = get_lut();
		let mut key_trigger = ShortcutTriggerBuilder::<_>::new(lut)
			.with_shortcut("Ctrl+Alt+Key2".to_owned(), Box::new(|| println!("set unvisible")))
			.with_shortcut("Ctrl+Alt+Key1".to_owned(), Box::new(|| ()))
			.build()
			.unwrap();
		Application{key_trigger:std::cell::RefCell::new(key_trigger), name:"screenshoter".to_owned()}
	}

	pub fn get_app_name(&self)->String{
		self.name.clone()
	}

	pub fn on_init(&mut self){

	}

	pub fn on_keyboard_event(&self, input: KeyCode){
		self.key_trigger.borrow_mut().trigger(input);
	}

	pub fn on_update(&self){

	}
}

