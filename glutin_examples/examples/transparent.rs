#![allow(unused)]
mod app;

use app::{
    action,
    application::{Application, ApplicationBuilder},
    canvas::Canvas,
};

mod misc;

use glutin::{
    dpi::{LogicalPosition, LogicalSize, PhysicalSize, Position},
    event::{
        ElementState, Event, KeyboardInput, ModifiersState, MouseButton, VirtualKeyCode,
        WindowEvent,
    },
    event_loop::{ControlFlow, EventLoop},
    monitor::MonitorHandle,
    platform::run_return::EventLoopExtRunReturn,
    window::{Fullscreen, Window, WindowBuilder},
    ContextBuilder, ContextWrapper, NotCurrent, WindowedContext,
};

use raw_window_handle::{HasRawWindowHandle, RawWindowHandle, Win32Handle};

use windows::{
    core::*, Data::Xml::Dom::*, Win32::Foundation::*, Win32::System::Threading::*,
    Win32::UI::WindowsAndMessaging::*,
};

fn main_loop() {
    let mut el = EventLoop::new();
    let monitor = el.available_monitors().nth(0).expect("Invalid monitor handle");

    let wb = WindowBuilder::new()
        .with_decorations(false)
        .with_transparent(true)
        .with_maximized(true)
        .with_always_on_top(false);

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

    let app = ApplicationBuilder::new().with_name("EasyCapture".to_owned()).build().unwrap();

    // init canvas
    let canvas = Canvas::new(&windowed_context, monitor);

    app.on_init(windowed_context.window().raw_window_handle());
    el.run_return(|event, _, control_flow| {
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
                    if state == ElementState::Pressed {
                        app.mouse_begin = app.mouse_prev_pos;
                    }
                }
                _ => (),
            },
            Event::UserEvent(()) => {}
            Event::RedrawRequested(_) => {
                println!("{:?}", app.mouse_begin);
                if app.mouse_state == ElementState::Pressed {
                    canvas.on_draw();
                }
            }
            _ => (),
        }
    });
}

fn main() {
    main_loop();
}
