#![allow(unused)]
mod support;
use app::application::Application;
use glutin::dpi::{LogicalPosition, LogicalSize, PhysicalSize, Position};
use glutin::event::{Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::monitor::MonitorHandle;
use glutin::window::{Fullscreen, Window, WindowBuilder};
use glutin::{ContextBuilder, ContextWrapper, NotCurrent, WindowedContext};

mod misc;
use misc::shortcutkey::{get_lut, ShortcutTrigger, ShortcutTriggerBuilder, State};

mod app;
mod window_system;
use window_system::{glutin::GlutinSystem, WindowSystem};

fn main() {
    window_system::glutin::GlutinSystem::new().run(Application::new());
}
