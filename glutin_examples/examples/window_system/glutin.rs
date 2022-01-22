use super::WindowSystem;

use crate::app::application::Application;

use crate::support;

use crate::misc::KeyCode;

use crate::app::graphics_impl::opengl_impl::{GraphicsOpenGLImpl};

use glutin::dpi::{LogicalPosition, LogicalSize, PhysicalSize, Position};
use glutin::event::{Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::monitor::MonitorHandle;
use glutin::window::{Fullscreen, Window, WindowBuilder};
use glutin::{ContextBuilder, NotCurrent, WindowedContext, ContextWrapper};

pub struct GlutinSystem{
	el:EventLoop<()>,
}

impl GlutinSystem{
	pub fn new()->GlutinSystem{
		GlutinSystem{el:EventLoop::new()}
	}
}

fn translate_keycode(keycode:VirtualKeyCode)->Option<KeyCode>{
	match keycode{
		VirtualKeyCode::Key0=>Some(KeyCode::Key0),
		VirtualKeyCode::Key1=>Some(KeyCode::Key1),
		VirtualKeyCode::Key2=>Some(KeyCode::Key2),
		VirtualKeyCode::Key3=>Some(KeyCode::Key3),
		VirtualKeyCode::Key4=>Some(KeyCode::Key4),
		VirtualKeyCode::Key5=>Some(KeyCode::Key5),
		VirtualKeyCode::Key6=>Some(KeyCode::Key6),
		VirtualKeyCode::Key7=>Some(KeyCode::Key7),
		VirtualKeyCode::Key8=>Some(KeyCode::Key8),
		VirtualKeyCode::Key9=>Some(KeyCode::Key9),
		VirtualKeyCode::LControl=>Some(KeyCode::LControl),
		VirtualKeyCode::LAlt=>Some(KeyCode::LAlt),
		_=>None
	}
}

impl WindowSystem for GlutinSystem{
	fn run(self, app: Application)->! {

		let mut app = app;
		let monitor = self.el.available_monitors().nth(0).expect("Invalid monitor handle");
		let desktop_size = monitor.size();
		let wb = WindowBuilder::new().with_decorations(true).with_transparent(true);

		let windowed_context = ContextBuilder::new()
			.with_gl_profile(glutin::GlProfile::Core)
			.build_windowed(wb, &self.el)
			.unwrap();

		let windowed_context = unsafe { windowed_context.make_current().unwrap() };

		windowed_context.window().set_outer_position(LogicalPosition::new(0, 0));

    	println!("Pixel format of the window's GL context: {:?}", windowed_context.get_pixel_format());

    	let render_api = support::load(&windowed_context.context());

		let graphics = GraphicsOpenGLImpl::new(render_api, (desktop_size.width, desktop_size.height));

		self.el.run(move|event,_,control_flow|{
			*control_flow = ControlFlow::Wait;
			//
			match event {
				Event::LoopDestroyed => return,
				Event::WindowEvent { event, .. } => match event {
					WindowEvent::Resized(physical_size) => windowed_context.resize(physical_size),
					WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
					WindowEvent::MouseInput { button, .. } => {},
					WindowEvent::ModifiersChanged(modifier)=>{
						app.on_modifier_state_changed(modifier);
					},
					WindowEvent::KeyboardInput {
						input: KeyboardInput { virtual_keycode: Some(virtual_code), state, .. },
						..
					} => match state {
						glutin::event::ElementState::Pressed => {
							if virtual_code == VirtualKeyCode::Escape {
								*control_flow = ControlFlow::Exit;
							} else {
								app.on_keyboard_event(virtual_code, );
							}
						}
						_ => (),
					},
					_ => (),
				},
				Event::UserEvent(())=>{

				},
				Event::RedrawRequested(_) => {
					// render_api.draw_frame([0.0, 0.0, 0.0, 0.0]);
					app.on_draw(&graphics);
					windowed_context.swap_buffers().unwrap();
					println!("present");
				}
				_ => (),
			}

		});
	}

}