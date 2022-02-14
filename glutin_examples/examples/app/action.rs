use chrono::Utc;
use directories::UserDirs;
use std::ops::Bound;
use std::path::PathBuf;

use super::event::WindowEventHandler;
use super::window::TargetId;
use super::{
    canvas::Bound2,
    capture::CaptureDevice,
    event::{Event, UserEvent},
    window::MainWindow,
};
use chrono::Timelike;
use glutin::{
    event::{ModifiersState, VirtualKeyCode},
    event_loop::EventLoopProxy,
};
use log::debug;
pub trait Execute<A: ActionContext> {
    fn execute(&self, ctx: &mut A);
}

#[derive(Debug, Clone, Copy)]
pub enum Action {
    ImageCapture,
    DoImageCapture(Bound2),
    GifCapture,
    Suspend,
}

impl<A> Execute<A> for Action
where
    A: ActionContext,
{
    fn execute(&self, ctx: &mut A) {
        match self {
            Self::ImageCapture => ctx.invoke_image_capture(),
            Self::GifCapture => ctx.invoke_gif_capture(),
            Self::Suspend => ctx.suspend(),
            Self::DoImageCapture(rect) => {
                ctx.do_image_capture(*rect);
                ctx.suspend();
            }
        }
    }
}

pub trait ActionContext {
    fn invoke_image_capture(&mut self);
    fn invoke_gif_capture(&mut self);
    fn do_image_capture(&mut self, rect: Bound2);
    fn suspend(&mut self);
}

pub struct AppContext<'a> {
    pub event_proxy: &'a mut EventLoopProxy<UserEvent>,
    pub main_window: &'a mut dyn WindowEventHandler,
    pub capture_device: &'a mut CaptureDevice,
}

impl<'a> ActionContext for AppContext<'a> {
    fn invoke_image_capture(&mut self) {
        debug!("invoke_image_capture");
        self.event_proxy
            .send_event(UserEvent::new(Some(TargetId::MainWindow), Event::InvokeRegionSelector));
    }

    fn invoke_gif_capture(&mut self) {
        debug!("invoke_gif_capture");
        self.event_proxy
            .send_event(UserEvent::new(Some(TargetId::MainWindow), Event::InvokeRegionSelector));
    }

    fn suspend(&mut self) {
        debug!("suspend");
        self.main_window.set_visible(false);
    }

    fn do_image_capture(&mut self, rect: Bound2) {
        debug!("do_image_capture");
        let image = self.capture_device.capture_image(rect);
        let user_dir = UserDirs::new();
        let mut desktop_dir = user_dir.desktop_dir().map_or(PathBuf::new(), |f| f.to_path_buf());
        let now = Utc::now();
        let ts = chrono::offset::Local::now().format("%F-%H-%M-%S").to_string();
        let filename = format!("CAP_{}.png", ts);
        desktop_dir.push(filename);
        image.save(desktop_dir).unwrap();
    }
}

pub struct KeyBinding<T: Eq> {
    pub action: Action,
    pub mods: ModifiersState,
    pub key: T,
}

impl<T: Eq> KeyBinding<T> {
    #[inline(always)]
    pub fn is_triggered(&self, mods: ModifiersState, key: T) -> bool {
        (self.mods == mods && self.key == key)
    }
}
