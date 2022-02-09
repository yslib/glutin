use std::path::PathBuf;

use glutin::dpi::{LogicalPosition, LogicalSize, PhysicalSize, Position};
use glutin::event::{
    ElementState, Event, KeyboardInput, ModifiersState, MouseButton, VirtualKeyCode, WindowEvent,
};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::monitor::MonitorHandle;
use glutin::window::{Fullscreen, WindowBuilder};
use glutin::ContextBuilder;
use raw_window_handle::RawWindowHandle;

use crate::misc::shortcutkey::{get_lut, ShortcutTrigger, ShortcutTriggerBuilder, State};

use super::graphics::Graphics;

pub struct ApplicationBuilder {
    app_name: String,
    config_file_path: PathBuf,
}

impl ApplicationBuilder {
    pub fn new() -> Self {
        ApplicationBuilder {
            app_name: "".to_owned(),
            config_file_path: PathBuf::from("".to_owned()),
        }
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.app_name = name;
        self
    }

    pub fn with_config_file_path<T: AsRef<PathBuf>>(mut self, path: T) -> Self {
        self.config_file_path = path.as_ref().to_owned();
        self
    }

    pub fn build(self) -> Result<Application, ()> {
        let app = Application {
            name: self.app_name.clone(),
            mods: ModifiersState::empty(),
            mouse_state: ElementState::Released,
            mouse_begin: From::from((0, 0)),
            mouse_pos: From::from((0, 0)),
            mouse_prev_pos:From::from((0, 0))
        };
        Ok(app)
    }
}

pub struct Application {
    name: String,

    pub mods: ModifiersState,
    pub mouse_state: ElementState,
    pub mouse_pos: LogicalPosition<i32>,
    pub mouse_begin: LogicalPosition<i32>,
    pub mouse_prev_pos: LogicalPosition<i32>
}

struct AppData {}

pub struct Image {}

pub trait CaptureDeviceContext {
    fn capture(&self) -> Image;
}

unsafe impl Send for Application {}
unsafe impl Sync for Application {}

impl Application {
    pub fn on_init(&mut self, native_window: RawWindowHandle) {
        //
        //
    }

    pub fn handle_keyboard_event(&self, input: KeyboardInput) {}

    pub fn handle_mouse_event(&self, event: WindowEvent) {
        println!("handle_mouse_event: {:?}", event);
    }

    pub fn mouse_press_event(&self) {}

    pub fn on_modifier_state_changed(&mut self, modifier: ModifiersState) {
        self.mods = modifier;
    }

    pub fn on_update(&self) {}

    pub fn on_draw(&self, graphics: &dyn Graphics) {
        graphics.draw_rect(100, 100, 100, 100);
    }

    pub fn on_app_close(&self) {}
}
