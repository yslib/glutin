use super::window::WindowId;
use super::{
    capture::CaptureDevice,
    event::{Event, UserEvent},
    window::MainWindow,
};
use glutin::{
    event::{ModifiersState, VirtualKeyCode},
    event_loop::EventLoopProxy,
};
pub trait Execute<A: ActionContext> {
    fn execute(&self, ctx: &mut A);
}

#[derive(Debug, Clone, Copy)]
pub enum Action {
    ImageCapture,
    DoImageCapture,
    GifCapture,
    Suspend,
}

impl<A> Execute<A> for Action
where
    A: ActionContext,
{
    fn execute(&self, ctx: &mut A) {
        match self {
            Self::ImageCapture => {
                ctx.invoke_image_capture();
            }
            Self::GifCapture => {
                ctx.invoke_gif_capture();
            }
            Self::Suspend => {
                ctx.suspend();
            }
            Self::DoImageCapture => {
                ctx.do_image_capture();
            }
        }
    }
}

pub trait ActionContext {
    fn invoke_image_capture(&mut self);
    fn invoke_gif_capture(&mut self);
    fn do_image_capture(&mut self);
    fn suspend(&mut self);
}

pub struct AppContext<'a> {
    pub event_proxy: &'a mut EventLoopProxy<UserEvent>,
    pub main_window: &'a mut MainWindow,
    pub capture_device: &'a mut CaptureDevice,
}

impl<'a> ActionContext for AppContext<'a> {
    fn invoke_image_capture(&mut self) {
        println!("invoke_image_capture");
        self.event_proxy
            .send_event(UserEvent::new(Some(WindowId::MainWindow), Event::InvokeRegionSelector));
    }

    fn invoke_gif_capture(&mut self) {
        println!("invoke_gif_capture");
        self.event_proxy
            .send_event(UserEvent::new(Some(WindowId::MainWindow), Event::InvokeRegionSelector));
    }

    fn suspend(&mut self) {
        println!("suspend");
        self.main_window.set_visible(false);
    }

    fn do_image_capture(&mut self) {
        let rect = self.main_window.get_selector_region();
        self.capture_device.capture_image(rect);
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
