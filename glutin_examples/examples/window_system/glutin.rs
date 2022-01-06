use super::WindowSystem;

use crate::app::application::Application;

use glutin::dpi::{LogicalPosition, LogicalSize, PhysicalSize, Position};
use glutin::event::{Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::monitor::MonitorHandle;
use glutin::window::{Fullscreen, Window, WindowBuilder};
use glutin::{ContextBuilder, NotCurrent, WindowedContext, ContextWrapper};

pub struct GlutinSystem{
	el:EventLoop<()>,
}


impl WindowSystem for GlutinSystem{
	fn run(self, app:Application)->! {

		let monitor = self.el.available_monitors().nth(0).expect("Invalid monitor handle");
		let desktop_size = monitor.size();
		let wb = WindowBuilder::new().with_decorations(true).with_transparent(true);
		self.el.run(move|_,_,_|{

		});
	}
}