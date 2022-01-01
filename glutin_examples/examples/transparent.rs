mod support;

use glutin::dpi::{Size, PhysicalSize};

use glutin::event::{Event, WindowEvent, KeyboardInput, VirtualKeyCode};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::monitor::MonitorHandle;

use glutin::window::{WindowBuilder, Fullscreen};
use glutin::ContextBuilder;

fn main() {
    let el = EventLoop::new();
    let monitor = el.available_monitors().nth(0).expect("Invalid monitor handle");
    let wb = WindowBuilder::new()
        .with_title("ScreenShoter")
        .with_decorations(true)
        .with_transparent(true)
        .with_fullscreen(Some(Fullscreen::Borderless(Some(monitor))));

    let windowed_context = ContextBuilder::new().build_windowed(wb, &el).unwrap();

    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    println!("Pixel format of the window's GL context: {:?}", windowed_context.get_pixel_format());

    let gl = support::load(&windowed_context.context());

    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => windowed_context.resize(physical_size),
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput{input: KeyboardInput{virtual_keycode:Some(virtual_code), state, ..}, ..}=>match (virtual_code, state){
                    (VirtualKeyCode::Escape, _)=>{
                        windowed_context.window().set_visible(false);
                    },
                    (VirtualKeyCode::Key5, _)=>{
                        windowed_context.window().set_visible(true);
                    }
                    _=>()
                }
                _ => (),
            },
            Event::RedrawRequested(_) => {
                gl.draw_frame([0.0, 0.0, 0.0, 0.5]);
                windowed_context.swap_buffers().unwrap();
            },
            _ => (),
        }
    });
}
