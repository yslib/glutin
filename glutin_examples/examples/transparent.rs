#![allow(unused)]

mod app;
use app::action;
use app::application::{Application, ApplicationBuilder};

mod misc;
mod support;

use glutin::dpi::{LogicalPosition, LogicalSize, PhysicalSize, Position};
use glutin::event::{ElementState, Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::monitor::MonitorHandle;
use glutin::platform::ContextTraitExt;
use glutin::window::{Fullscreen, Window, WindowBuilder, *};
use glutin::{ContextBuilder, ContextWrapper, NotCurrent, WindowedContext};

use raw_window_handle::{HasRawWindowHandle, RawWindowHandle, Win32Handle};

use crate::app::graphics_impl::opengl_impl::GraphicsOpenGLImpl;

use windows::{
    core::*, Data::Xml::Dom::*, Win32::Foundation::*, Win32::System::Threading::*,
    Win32::UI::WindowsAndMessaging::*,
};

use app::graphics::Graphics;

struct Bound2 {
    pub min: (i32, i32),
    pub max: (i32, i32),
}

impl Bound2 {
    fn new(p1: (i32, i32), p2: (i32, i32)) -> Self {
        Bound2 {
            min: (std::cmp::min(p1.0, p2.0), std::cmp::min(p1.1, p2.1)),
            max: (std::cmp::max(p1.0, p2.0), std::cmp::max(p1.1, p2.1)),
        }
    }

    fn rect(&self) -> (i32, i32, u32, u32) {
        let min = self.min;
        let max = self.max;
        (min.0, min.1, (max.0 - min.0) as u32, (max.1 - min.1) as u32)
    }
}

fn main_loop(app: Application) -> ! {
    let el = EventLoop::new();
    let mut app = app;
    let monitor = el.available_monitors().nth(1).expect("Invalid monitor handle");
    let desktop_size = monitor.size();
    let wb =
        WindowBuilder::new().with_decorations(false).with_transparent(true).with_maximized(true);
    //.with_always_on_top(true);

    let windowed_context = ContextBuilder::new()
        .with_gl_profile(glutin::GlProfile::Core)
        .build_windowed(wb, &el)
        .unwrap();

    let windowed_context = unsafe { windowed_context.make_current().unwrap() };
    windowed_context.window().set_outer_position(LogicalPosition::new(0, 0));

    if cfg!(windows) {
        let handle = windowed_context.window().raw_window_handle();
        unsafe {
            match handle {
                RawWindowHandle::Win32(Win32Handle { hwnd, hinstance, .. }) => {
                    let hwnd = HWND(hwnd as isize);
                    let mut exstyle = WINDOW_EX_STYLE(GetWindowLongPtrW(hwnd, GWL_EXSTYLE) as u32);
                    exstyle = exstyle | WS_EX_TOOLWINDOW;
                    SetWindowLongW(hwnd, GWL_EXSTYLE, exstyle.0 as i32);
                }
                _ => (),
            }
        }
    }

    let graphics = GraphicsOpenGLImpl::new((desktop_size.width, desktop_size.height), &windowed_context);

    app.on_init(windowed_context.window().raw_window_handle());
    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        //
        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => windowed_context.resize(physical_size),
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::ModifiersChanged(modifier) => {
                    println!("ModifiersChanged");
                    app.mods = modifier;
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    app.handle_keyboard_event(input);
                }
                WindowEvent::CursorMoved { position, .. } => {
                    app.mouse_prev_pos = From::from((position.x, position.y));
                    if app.mouse_state == ElementState::Pressed {
                        app.mouse_pos = From::from((position.x, position.y));
                        windowed_context.window().request_redraw();
                    }
                }
                WindowEvent::MouseInput { state, button, .. } => {
                    app.mouse_state = state;
                    if state == ElementState::Pressed{
                        app.mouse_begin = app.mouse_prev_pos;
                    }
                }
                _ => (),
            },
            Event::UserEvent(()) => {}
            Event::RedrawRequested(_) => {
                // graphics.clear((0f32, 0f32, 0f32, 0.1f32));
                // graphics.draw_region(100, 100, 100, 100);
                println!("{:?}", app.mouse_begin);
                if app.mouse_state == ElementState::Pressed {
                    let rect =
                        Bound2::new(From::from(app.mouse_begin), From::from(app.mouse_pos)).rect();
                    graphics.clear((0.0,0.0,0.0,0.0));
                    graphics.draw_rect(rect.0,rect.1,rect.2,rect.3);
                    windowed_context.swap_buffers().unwrap();
                }
            }
            _ => (),
        }
    });
}

fn main() {
    let app = ApplicationBuilder::new().with_name("EasyCapture".to_owned()).build().unwrap();

    main_loop(app);
}
