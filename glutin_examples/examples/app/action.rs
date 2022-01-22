use bitflags::bitflags;

use crate::misc::KeyCode;

bitflags! {
    struct Modifiers:u32{
        const CTRL = 0b100;
        const ALT = 0b100 << 3;
        const SHIFT = 0b100 << 6;
        const LOGO = 0b100<<9;
    }
}
pub trait Execute<A:ActionContext>{
    fn execute(self, ctx: &mut A);
}

pub enum Action{
    ImageCapture,
    GifCapture
}

impl<A> Execute<A> for Action where A:ActionContext{
    fn execute(self, ctx: &mut A) {
        match self {
            Self::ImageCapture=>{
                ctx.capture_image();
            },
            Self::GifCapture=>{
                ctx.capture_gif();
            }
        }
    }
}

pub trait ActionContext{
    fn capture_image(&mut self);
    fn capture_gif(&mut self);
}

pub struct KeyBinding<T:Eq>{
    action:Action,
    mods :Modifiers,
    key: T,
}