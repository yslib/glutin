use glutin::{event::{ModifiersState, VirtualKeyCode}, event_loop::EventLoopProxy};
use super::event::UserEvent;
pub trait Execute<A: ActionContext> {
    fn execute(&self, ctx: &mut A);
}


pub enum Action<E>{
    ImageCapture,
    GifCapture,
    UserAction(E)
}

impl<A, E> Execute<A> for Action<E>
where
    A: ActionContext
{
    fn execute(&self, ctx: &mut A) {
        match self {
            Self::ImageCapture => {
                ctx.capture_image();
            }
            Self::GifCapture => {
                ctx.capture_gif();
            }
            Self::UserAction(e)=>{
            }
        }
    }
}

pub trait ActionContext {
    fn capture_image(&mut self);
    fn capture_gif(&mut self);
}

pub struct AppContext<'a, T:'static>{
    pub event_proxy:&'a mut EventLoopProxy<T>
}

impl<'a, T> ActionContext for AppContext<'a, T>{
    fn capture_image(&mut self){
        println!("capture_image");
        self.event_proxy.send_event(T::from(13));
    }

    fn capture_gif(&mut self){

    }
}

pub struct KeyBinding<T: Eq, E> {
    pub action: Action<E>,
    pub mods: ModifiersState,
    pub key: T,
}

impl<T:Eq, E> KeyBinding<T, E>{

    #[inline(always)]
    pub fn is_triggered(&self, mods: ModifiersState, key:T)->bool{
        (self.mods == mods && self.key == key)
    }
}