use glutin::{
    dpi::{LogicalPosition, PhysicalPosition},
    event::{ElementState, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent},
};

use super::{action::Action, window::TargetId};

#[derive(Debug)]
pub struct MouseData {
    pub button: MouseButton,
    pub position: PhysicalPosition<f64>,
}

#[derive(Debug)]
pub struct KeyInputData {
    pub virtual_keycode: VirtualKeyCode,
}

#[derive(Debug, Clone, Copy)]
pub enum Event {
    InvokeRegionSelector,
    DoAction(Action),
}

#[derive(Debug, Clone, Copy)]
pub struct UserEvent {
    pub window_id: Option<TargetId>,
    pub event: Event,
}

impl UserEvent {
    pub fn new(window_id: Option<TargetId>, event: Event) -> Self {
        Self { window_id, event }
    }
}

pub trait WindowEventHandler {
    fn on_mouse_press_event(&mut self, data: &MouseData);

    fn on_mouse_release_event(&mut self, data: &MouseData);

    fn on_mouse_move_event(&mut self, data: &MouseData);

    fn on_keyboard_event(&mut self, data: &KeyInputData);

    fn handle_redraw_event(&mut self);

    fn on_user_event(&mut self, data: &UserEvent);

    fn set_visible(&mut self, visible: bool);
}
