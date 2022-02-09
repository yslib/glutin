use bitflags::bitflags;
use glutin::event::ModifiersState;
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
    mods :ModifiersState,
    key: T,
}