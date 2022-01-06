#![allow(unused)]
mod support;
use glutin::dpi::{LogicalPosition, LogicalSize, PhysicalSize, Position};
use glutin::event::{Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::monitor::MonitorHandle;
use glutin::window::{Fullscreen, Window, WindowBuilder};
use glutin::{ContextBuilder, NotCurrent, WindowedContext, ContextWrapper};

mod misc;
use misc::shortcutkey::{get_lut, ShortcutTrigger, ShortcutTriggerBuilder, State};

mod app;
mod window_system;


fn main() {
    let el = EventLoop::new();
    let monitor = el.available_monitors().nth(0).expect("Invalid monitor handle");
    let desktop_size = monitor.size();
    let wb = WindowBuilder::new().with_decorations(true).with_transparent(true);
    //.with_fullscreen(Some(Fullscreen::Borderless(Some(monitor))));
    //

    let windowed_context = ContextBuilder::new()
        .with_gl_profile(glutin::GlProfile::Core)
        .build_windowed(wb, &el)
        .unwrap();

    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    windowed_context.window().set_outer_position(LogicalPosition::new(0, 0));

    println!("Pixel format of the window's GL context: {:?}", windowed_context.get_pixel_format());

    let render_api = support::load(&windowed_context.context());

    let lut = get_lut();
    let cb = || println!("aaaa");

    let mut trigger = ShortcutTriggerBuilder::<(), _>::new(lut)
        .with_shortcut("Ctrl+Alt+Key2".to_owned(), Box::new(|| println!("set unvisible")))
        .with_shortcut("Ctrl+Alt+Key1".to_owned(), Box::new(|| ()))
        .build()
        .unwrap();

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
                } => match state {
                    glutin::event::ElementState::Pressed => {
                        if virtual_code == VirtualKeyCode::Escape {
                            *control_flow = ControlFlow::Exit;
                        } else {
                            trigger.trigger(virtual_code);
                        }
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
