mod support;

use glutin::dpi::{LogicalPosition, LogicalSize, PhysicalSize, Position};

use glutin::event::{Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::monitor::MonitorHandle;

use glutin::window::{Fullscreen, WindowBuilder};
use glutin::ContextBuilder;

pub mod state {
    use std::collections::HashMap;
    use std::hash::Hash;
    use std::marker::PhantomData;

    use glutin::event::VirtualKeyCode;

    #[derive(Clone, Hash, PartialEq, Eq)]
    pub enum State<T>
    where
        T: Clone + Hash + Eq,
    {
        Empty,
        State(T),
        Error,
        Accept,
    }

    pub trait Event
    where
        Self: Hash + Eq + Copy,
    {
    }

    pub struct Trans<'a, S>
    where
        S: Clone + Hash + Eq,
    {
        pub state: State<S>,
        pub callback: Option<Box<dyn FnMut() + 'a>>,
    }

    pub type Inner<'a, S, E> = HashMap<E, Trans<'a, S>>;
    pub type TransTable<'a, S, E> = HashMap<State<S>, Inner<'a, S, E>>;

    pub struct ShortcutTrigger<'a, S, E>
    where
        S: Clone + Hash + Eq,
        E: Event,
    {
        table: TransTable<'a, S, E>,
        current_state: State<S>,
    }

    impl<'a, S, E> ShortcutTrigger<'a, S, E>
    where
        S: Clone + Hash + Eq,
        E: Event,
    {
        pub fn new(table: TransTable<'a, S, E>, initial: State<S>) -> Self {
            ShortcutTrigger { table, current_state: initial }
        }

        pub fn trigger(&mut self, event: E) {
            if let Some(trans) = self.table.get_mut(&self.current_state) {
                if let Some(inner) = trans.get_mut(&event) {
                    self.current_state = inner.state.clone();
                    if let Some(cb) = inner.callback.as_mut() {
                        cb();
                    }
                    match self.current_state {
                        State::Accept => self.reset(),
                        _ => (),
                    }
                }
            }
        }

        pub fn reset(&mut self) {
            self.current_state = State::Empty;
        }
    }


    pub fn get_lut()->HashMap<String, VirtualKeyCode>{
        let lut = HashMap::from([
            ("Ctrl".to_string(), VirtualKeyCode::LControl),
            ("Alt".to_string(), VirtualKeyCode::LAlt),
            ("Key1".to_string(), VirtualKeyCode::Key1),
            ("Key2".to_string(), VirtualKeyCode::Key2),
            ("Key3".to_string(), VirtualKeyCode::Key3),
            ("Key4".to_string(), VirtualKeyCode::Key4),
            ("Key5".to_string(), VirtualKeyCode::Key5),
            ("Key6".to_string(), VirtualKeyCode::Key6),
        ]);
    lut
    }
    struct ShortcutTriggerBuilder<'a, T, E> {
        shortcuts: Vec<(String, Box<dyn FnMut() + 'a>)>,
        lut:HashMap<String, E>,
        phantom: PhantomData<&'a T>,
    }

    impl<'a, T, E> ShortcutTriggerBuilder<'a, T, E> where E:Event{
        pub fn new(dict:HashMap<String, E>)->ShortcutTriggerBuilder<'a, T, E>{
            ShortcutTriggerBuilder{
                shortcuts:vec![],
                lut:dict,
                phantom:PhantomData
            }
        }
        fn with_shortcut(mut self, shortcut: String, trigger: Box<dyn FnMut() + 'a>) -> Self {
            self.shortcuts.push((shortcut, trigger));
            self
        }
        fn build(self) -> Result<ShortcutTrigger<'a, String, E>, ()>
        where
            E: Event,
        {
            let mut table = TransTable::from([
                (State::Empty, Inner::<'a, String, E>::new())
            ]);
            for shortcut in self.shortcuts {
                let splits: Vec<_> = shortcut.0.split('+').collect();
                let cb = shortcut.1;
                let mut trans_pair = Vec::new();
                let mut unique_state = String::new();
                for (index, &s) in splits.iter().enumerate() {
                    let &trigger = self.lut.get(s).ok_or(())?;
                    unique_state += s;
                    if index == splits.len() - 1 {
                        trans_pair.push((trigger, State::Accept, None));
                    } else {
                        trans_pair.push((trigger, State::State(unique_state.clone()), None));
                    }
                }


                let mut state = State::Empty;
                for trans in trans_pair{
                    let f = |e:&mut Inner<'a, String, E>|{
                        e.insert(trans.0, Trans{state:trans.1.clone(), callback:trans.2});
                    };
                    table.entry(state)
                    .and_modify(|e|f(e))
                    .or_insert_with(||{
                        let mut new = Inner::new();
                        f(&mut new);
                        new
                    });
                    state = trans.1.clone();
                }
            }
            Ok(ShortcutTrigger { table:table, current_state: State::Empty })
        }
    }
}

#[cfg(test)]
mod test {
    use super::state::ShortcutTrigger;
    use crate::state::Event;
    use crate::state::State;
    use crate::state::Trans;
    use crate::state::TransTable;
    use glutin::event::VirtualKeyCode;
    use std::collections::HashMap;
    use std::str::FromStr;

    impl Event for VirtualKeyCode {}
    impl State for String {}

    #[test]
    fn state_machine_test() {
        let table = HashMap::from([(
            "null".to_owned(),
            HashMap::from([(
                VirtualKeyCode::Key0,
                Trans {
                    state: "screenshot".to_owned(),
                    callback: Box::new(|| println!("screenshot")),
                },
            )]),
        )]);

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

    windowed_context.window().set_outer_position(LogicalPosition::new(0, 0));

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
                WindowEvent::MouseInput { button, .. } => {}
                WindowEvent::KeyboardInput {
                    input: KeyboardInput { virtual_keycode: Some(virtual_code), state, .. },
                    ..
                } => match (virtual_code, state) {
                    (VirtualKeyCode::Escape, _) => *control_flow = ControlFlow::Exit,
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
