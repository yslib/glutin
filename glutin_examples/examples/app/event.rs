use glutin::{
    dpi::{PhysicalPosition, LogicalPosition},
    event::{KeyboardInput, WindowEvent, ElementState, MouseButton, VirtualKeyCode},
};

#[derive(Debug)]
pub struct MouseData{
	pub button: MouseButton,
	pub position: PhysicalPosition<f64>,
}

#[derive(Debug)]
pub struct KeyInputData{
	pub virtual_keycode:VirtualKeyCode,
}


#[derive(Debug)]
pub enum UserEvent{
	InvokeRegionSelector
}


pub trait WindowEventHandler<T> {
	fn on_mouse_press_event(&self, data: &MouseData);

	fn on_mouse_release_event(&self, data: &MouseData);

	fn on_mouse_move_event(&self, data: &MouseData);

	fn on_keyboard_event(&self, data: &KeyInputData);

	fn on_user_event(&self, data: &T);
}