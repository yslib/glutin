use glutin::{
    event::{KeyboardInput, MouseScrollDelta, WindowEvent},
    event_loop::EventLoopProxy,
    window::Window,
    ContextWrapper, PossiblyCurrent, WindowedContext,
};

use super::{
    action::Action,
    canvas::{Bound2, Canvas, RegionSelector, Renderable},
    event::{KeyInputData, MouseData, UserEvent, WindowEventHandler},
    graphics::Graphics,
};

use log::{debug, info};

#[derive(Debug, Clone, Copy)]
pub enum TargetId {
    Application,
    MainWindow,
    Action,
}

pub struct MainWindow {
    pub windowed_context: Option<ContextWrapper<PossiblyCurrent, Window>>,
    pub graphics: Box<dyn Graphics>,
    pub event_proxy: EventLoopProxy<UserEvent>,
    pub region_selector: RegionSelector,
    window_id: TargetId,
}

pub trait IWindow {
    fn get_window_id(&self) -> TargetId;
}

impl IWindow for MainWindow {
    fn get_window_id(&self) -> TargetId {
        self.window_id
    }
}

impl MainWindow {
    pub fn new(
        windowed_context: WindowedContext<PossiblyCurrent>,
        graphics: Box<dyn Graphics>,
        event_proxy: EventLoopProxy<UserEvent>,
        window_id: TargetId,
    ) -> Self {
        MainWindow {
            windowed_context: Some(windowed_context),
            graphics,
            event_proxy,
            window_id,
            region_selector: RegionSelector::new(),
        }
    }
}

impl MainWindow {
    pub fn swap_buffers(&self) {
        self.windowed_context.as_ref().unwrap().swap_buffers().expect("swap buffer");
    }

    pub fn make_current(&mut self) {
        if !self.windowed_context.as_ref().unwrap().is_current() {
            self.windowed_context = Some(unsafe {
                self.windowed_context.take().unwrap().make_current().expect("context swap")
            });
        }
    }

    pub fn request_redraw(&self) {
        self.windowed_context.as_ref().unwrap().window().request_redraw();
    }

    pub fn get_selector_region(&self) -> Bound2 {
        self.region_selector.bound
    }
}

impl WindowEventHandler for MainWindow {
    fn on_mouse_press_event(&mut self, data: &MouseData) {
        self.region_selector.set_visible(true);
        self.region_selector.set_first(data.position.into());
    }

    fn on_mouse_release_event(&mut self, data: &MouseData) {
        self.region_selector.set_visible(false);
        let do_capture_event = UserEvent {
            window_id: Some(TargetId::Action),
            event: crate::app::event::Event::DoAction(Action::DoImageCapture(
                self.region_selector.bound,
            )),
        };
        self.event_proxy.send_event(do_capture_event);
        self.request_redraw();
    }

    fn on_mouse_move_event(&mut self, data: &MouseData) {
        self.region_selector.set_second(data.position.into());
        self.request_redraw();
    }

    fn on_keyboard_event(&mut self, data: &KeyInputData) {
        // unimplemented!();
    }

    fn handle_redraw_event(&mut self) {
        self.graphics.clear((0.0, 0.0, 0.0, 0.5));
        self.region_selector.update(&*self.graphics);
        self.swap_buffers();
    }

    fn on_user_event(&mut self, data: &UserEvent) {
        match data.event {
            crate::app::event::Event::InvokeRegionSelector => {
                self.set_visible(true);
            }
            _ => {}
        }
    }

    fn set_visible(&mut self, visible: bool) {
        self.region_selector.bound = Bound2::default();
        println!("{:?}", self.region_selector.bound);
        self.windowed_context.as_ref().map(|f| {
            info!("set main window visible: {}", visible);
            f.window().set_visible(visible);
        });
    }
}
