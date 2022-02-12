use std::rc::Rc;
use std::{marker::PhantomData, path::PathBuf};

use crate::support;

use glutin::dpi::PhysicalPosition;
use glutin::event_loop::EventLoopProxy;
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
    ContextBuilder, ContextWrapper, NotCurrent, PossiblyCurrent,
};

use super::{
    action::{Action, ActionContext, AppContext, Execute, KeyBinding},
    canvas::{Canvas, RegionSelector},
    event::{KeyInputData, MouseData, WindowEventHandler},
    graphics::Graphics,
    graphics_impl::opengl_impl::GraphicsOpenGLImpl,
    window::MainWindow,
};

use raw_window_handle::{HasRawWindowHandle, RawWindowHandle, Win32Handle};

use windows::{
    core::*, Data::Xml::Dom::*, Win32::Foundation::*, Win32::System::Threading::*,
    Win32::UI::WindowsAndMessaging::*,
};

pub struct ApplicationBuilder<T> {
    app_name: String,
    config_file_path: PathBuf,
    phantom: PhantomData<T>,
}

impl<T> ApplicationBuilder<T> {
    pub fn new() -> Self {
        ApplicationBuilder {
            app_name: "".to_owned(),
            config_file_path: PathBuf::from("".to_owned()),
            phantom: PhantomData,
        }
    }
    pub fn with_name(mut self, name: &str) -> Self {
        self.app_name = name.to_owned();
        self
    }

    pub fn with_config_file_path<U: AsRef<PathBuf>>(mut self, path: U) -> Self {
        self.config_file_path = path.as_ref().to_owned();
        self
    }

    fn platform_config(&self, windowed_context: &ContextWrapper<PossiblyCurrent, Window>) {
        if cfg!(windows) {
            let handle = windowed_context.window().raw_window_handle();
            unsafe {
                match handle {
                    RawWindowHandle::Win32(Win32Handle { hwnd, hinstance, .. }) => {
                        let hwnd = HWND(hwnd as isize);
                        let mut exstyle =
                            WINDOW_EX_STYLE(GetWindowLongPtrW(hwnd, GWL_EXSTYLE) as u32);
                        exstyle = exstyle | WS_EX_TOOLWINDOW;
                        SetWindowLongW(hwnd, GWL_EXSTYLE, exstyle.0 as i32);
                        println!("set window GLW_EXSTYLE")
                    }
                    _ => (),
                }
            }
        }
    }

    fn load_keybinding_actions(&self) -> Vec<KeyBinding<VirtualKeyCode, T>> {
        vec![KeyBinding {
            action: Action::<T>::ImageCapture,
            mods: ModifiersState::CTRL | ModifiersState::ALT,
            key: VirtualKeyCode::Key1,
        }]
    }

    fn create_graphics<'a, 'b>(
        &self,
        windowed_context: &'a ContextWrapper<PossiblyCurrent, Window>,
        event_loop: &'b EventLoop<T>,
    ) -> Box<dyn Graphics> {
        use std::cell::RefCell;

        let monitor = event_loop.available_monitors().nth(0).expect("Invalid monitor handle");
        let size = monitor.size();
        let render_api = support::load(windowed_context);
        Box::new(GraphicsOpenGLImpl {
            render_api: RefCell::new(render_api),
            desktop_size: (size.width, size.height),
        })
    }

    fn create_window_context(
        &self,
        event_loop: &EventLoop<T>,
    ) -> ContextWrapper<NotCurrent, Window> {
        let wb = WindowBuilder::new()
            .with_title(self.app_name.clone())
            .with_decorations(true)
            .with_transparent(true)
            .with_maximized(false)
            .with_always_on_top(false);

        let windowed_context = ContextBuilder::new()
            .with_gl_profile(glutin::GlProfile::Core)
            .build_windowed(wb, event_loop)
            .unwrap();
        windowed_context
    }

    fn create_main_window(&self, event_loop: &EventLoop<T>) -> MainWindow<T> {
        let wb = WindowBuilder::new()
            .with_decorations(false)
            .with_transparent(true)
            .with_maximized(true)
            .with_always_on_top(false);

        let windowed_context = ContextBuilder::new()
            .with_gl_profile(glutin::GlProfile::Core)
            .build_windowed(wb, event_loop)
            .unwrap();

        let windowed_context = unsafe { windowed_context.make_current().expect("make current") };

        let graphics = self.create_graphics(&windowed_context, &event_loop);
        // let canvas = Canvas { objects: vec![], graphics };
        MainWindow {
            windowed_context: Some(windowed_context),
            graphics,
            event_proxy: event_loop.create_proxy(),
        }
    }

    pub fn build(self, event_loop: &EventLoop<T>) -> Result<Application<T>> {
        // let event_loop = EventLoop::<T>::with_user_event();
        let monitor = event_loop.available_monitors().nth(0).expect("Invalid monitor handle");
        let windowed_context = self.create_window_context(&event_loop);

        let windowed_context = unsafe { windowed_context.make_current().unwrap() };
        windowed_context.window().set_outer_position(LogicalPosition::new(0, 0));

        self.platform_config(&windowed_context);

        let main_window = self.create_main_window(event_loop);

        let app = Application {
            main_window,
            name: self.app_name.clone(),
            mods: ModifiersState::empty(),
            event_proxy:event_loop.create_proxy(),
            keybinding_actions: self.load_keybinding_actions(),
            mouse_state: ElementState::Released,
            mouse_begin: From::from((0, 0)),
            mouse_pos: From::from((0, 0)),
            mouse_prev_pos: From::from((0, 0)),
            mouse_btn: MouseButton::Left,
        };
        Ok(app)
    }
}

pub struct Application<T: 'static> {
    name: String,
    main_window: MainWindow<T>,
    keybinding_actions: Vec<KeyBinding<VirtualKeyCode, T>>,
    event_proxy: EventLoopProxy<T>,
    pub mods: ModifiersState,
    pub mouse_state: ElementState,
    pub mouse_pos: PhysicalPosition<f64>,
    pub mouse_begin: PhysicalPosition<f64>,
    pub mouse_prev_pos: PhysicalPosition<f64>,
    pub mouse_btn: MouseButton,
}

impl<T> Application<T> {
    pub fn handle_keyboard_event(&mut self, input: KeyboardInput) {
        input.virtual_keycode.map(|k| {
            let data = KeyInputData { virtual_keycode: k };
            self.main_window.on_keyboard_event(&data);
            // trigger

            let mut app_ctx = AppContext {
                event_proxy: &mut self.event_proxy
            };
            for binding in &self.keybinding_actions {
                if binding.is_triggered(self.mods, k) {
                    binding.action.execute(&mut app_ctx);
                }
            }
        });
    }

    pub fn handle_mouse_event(&mut self, event: WindowEvent) {
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_prev_pos = From::from((position.x, position.y));
                if self.mouse_state == ElementState::Pressed {
                    self.mouse_pos = From::from((position.x, position.y));
                    let mouse_data = MouseData { button: self.mouse_btn, position };
                    self.main_window.on_mouse_move_event(&mouse_data);
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                self.mouse_state = state;
                let mouse_data = MouseData { button, position: self.mouse_prev_pos };
                match state {
                    ElementState::Pressed => {
                        self.mouse_begin = self.mouse_prev_pos;
                        self.mouse_btn = button;
                        self.main_window.on_mouse_press_event(&mouse_data);
                    }
                    ElementState::Released => {
                        self.mouse_begin = self.mouse_prev_pos;
                        self.mouse_btn = button;
                        self.main_window.on_mouse_release_event(&mouse_data);
                    }
                }
            }
            _ => {
                panic!("unexpected mouse event")
            }
        }
    }

    pub fn handle_redraw_event(&mut self) {
        self.main_window.handle_redraw_event();
    }

    pub fn handle_user_event(&self, data: &T) {
        self.main_window.on_user_event(data);
    }

    pub fn on_modifier_state_changed(&mut self, modifier: ModifiersState) {
        self.mods = modifier;
    }

    pub fn run(&mut self, event_loop: EventLoop<T>) {
        let mut event_loop = event_loop;
        event_loop.run_return(|event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
            match event {
                Event::LoopDestroyed => return,
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::Resized(physical_size) => (),
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::ModifiersChanged(modifier) => {
                        self.mods = modifier;
                    }
                    WindowEvent::KeyboardInput { input, .. } => {
                        self.handle_keyboard_event(input);
                    }
                    WindowEvent::CursorMoved { .. } | WindowEvent::MouseInput { .. } => {
                        self.handle_mouse_event(event);
                    }
                    _ => (),
                },
                Event::UserEvent(t) => {
                    self.handle_user_event(&t);
                }
                Event::RedrawRequested(_) => {
                    self.handle_redraw_event();
                }
                _ => (),
            }
        });
    }
}
