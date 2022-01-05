use glutin::dpi::{LogicalPosition, LogicalSize, PhysicalSize, Position};
use glutin::event::{Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::monitor::MonitorHandle;
use glutin::window::{Fullscreen, WindowBuilder};
use glutin::ContextBuilder;

use crate::misc::shortcutkey::ShortcutTrigger;
pub struct Application<'a>{
	shortcut_key_trigger:ShortcutTrigger<'a, String, VirtualKeyCode>
}