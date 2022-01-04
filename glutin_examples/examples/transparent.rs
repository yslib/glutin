mod support;


use glutin::dpi::{LogicalSize, Position, LogicalPosition, PhysicalSize};

use glutin::event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent, MouseButton};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::monitor::MonitorHandle;

use glutin::window::{Fullscreen, WindowBuilder};
use glutin::ContextBuilder;



mod state {
    use std::hash::Hash;
    use std::collections::HashMap;
    use std::marker::PhantomData;

    use glutin::event::VirtualKeyCode;

    #[derive(Clone, Hash, PartialEq, Eq)]
    enum State<T> where T:Clone + Hash + Eq{
        Empty,
        State(T),
        Error,
        Accpet
    }

    pub trait Event where Self: Hash + Eq{}

    pub struct Trans<S> where S:Clone+Hash + Eq{
        pub state: State<S>,
        pub callback: Option<Box<dyn FnMut()>>
    }
    pub type TransTable<S, E> = HashMap<State<S>, HashMap<E, Trans<S>>>;

    pub struct ShortcutTrigger<S, E> where S: Clone+Hash+Eq, E:Event{
        table: TransTable<S, E>,
        current_state: State<S>
    }

    impl<S, E> ShortcutTrigger<S, E> where S:Clone+Hash+Eq, E:Event{
        pub fn new(table:TransTable<S, E>, initial:State<S>)->Self{
            ShortcutTrigger { table, current_state:initial}
        }

        pub fn trigger(&mut self, event:E){
            if let Some(trans) = self.table.get_mut(&self.current_state){
                if let Some(inner) = trans.get_mut(&event){
                    self.current_state = inner.state;
                    if let Some(cb)=inner.callback{
                        cb();
                    }
                    match self.current_state{
                        State::Accpet=>{
                            self.reset()
                        },
                        _=>()
                    }
                }
            }
        }

        pub fn reset(&mut self){
            self.current_state = State::Empty;
        }
    }

    struct ShortcutTriggerBuilder<'a, T>{
        shortcuts:Vec<(String, Box<dyn FnMut()>)>,
        phantom:PhantomData<&'a T>
    }

    impl<'a, T> ShortcutTriggerBuilder<'a, T>{
        fn with_shortcut(mut self, shortcut:String, trigger: Box<dyn FnMut()>)->Self{
            self.shortcuts.push((shortcut, trigger));
            return self
        }

        fn build<E>(self)->ShortcutTrigger<String, E> where E:Event{
            let lut = HashMap::from([
                ("Ctrl",VirtualKeyCode::LControl),
                ("Alt", VirtualKeyCode::LAlt),
                ("Key1", VirtualKeyCode::Key1),
                ("Key2", VirtualKeyCode::Key2),
                ("Key3", VirtualKeyCode::Key3),
            ]);
            let mut table = HashMap::new();
            for shortcut in self.shortcuts.into_iter(){
                let splits:Vec<_> = shortcut.0.split('+').collect();
                table.entry(State::Accept).or_insert(default)
            }
            ShortcutTrigger { table: table, current_state: State::Empty }
        }
    }
}


#[cfg(test)]
mod test{
    use glutin::event::VirtualKeyCode;
    use crate::state::Event;
    use crate::state::Trans;
    use crate::state::State;
    use crate::state::TransTable;
    use super::state::ShortcutTrigger;
    use std::collections::HashMap;
    use std::str::FromStr;

    impl Event for VirtualKeyCode{}
    impl State for String{}

    #[test]
    fn state_machine_test(){
        let table = HashMap::from([
            ("null".to_owned(), HashMap::from([
                (VirtualKeyCode::Key0, Trans{state:"screenshot".to_owned(), callback:Box::new(||println!("screenshot"))})
            ]))
        ]);

        let mut fa = ShortcutTrigger::new(table, "null".to_owned());
        fa.trigger(VirtualKeyCode::Key0);
    }

}


fn main() {
    let el = EventLoop::new();
    let monitor = el.available_monitors().nth(0).expect("Invalid monitor handle");
    let desktop_size = monitor.size();
    let wb = WindowBuilder::new()
        .with_decorations(true)
        .with_transparent(true)
        .with_fullscreen(Some(Fullscreen::Borderless(Some(monitor))));
        //

    let windowed_context = ContextBuilder::new()
        .with_gl_profile(glutin::GlProfile::Core)
        .build_windowed(wb, &el)
        .unwrap();

    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    windowed_context.window().set_outer_position(LogicalPosition::new(0,0));

    println!("Pixel format of the window's GL context: {:?}", windowed_context.get_pixel_format());

    let render_api = support::load(&windowed_context.context());

    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        //
        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => windowed_context.resize(physical_size),
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::MouseInput{button,..}=>
                {
                }
                WindowEvent::KeyboardInput {
                    input: KeyboardInput { virtual_keycode: Some(virtual_code), state, .. },
                    ..
                } => match (virtual_code, state) {
                    (VirtualKeyCode::Escape, _) => {
                        *control_flow = ControlFlow::Exit
                    }
                    (VirtualKeyCode::Key5, _) => {
                        windowed_context.window().set_visible(true);
                    }
                    (VirtualKeyCode::Key6, _) => {
                        windowed_context.window().set_visible(false);
                    }
                    _ => (),
                },
                _ => (),
            },
            Event::RedrawRequested(_) => {
                render_api.draw_frame([0.0, 0.0, 0.0, 0.0]);
                windowed_context.swap_buffers().unwrap();
                println!("present");
            }
            _ => (),
        }
    });
}
