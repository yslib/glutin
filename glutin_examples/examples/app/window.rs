
use glutin::{window::Window, ContextWrapper, PossiblyCurrent, event::{KeyboardInput, WindowEvent}, event_loop::EventLoopProxy};


use super::{canvas::Canvas, graphics::Graphics, event::{WindowEventHandler, MouseData, KeyInputData, UserEvent}};

pub struct MainWindow<T:'static> {
    pub windowed_context: Option<ContextWrapper<PossiblyCurrent, Window>>,
    pub graphics: Box<dyn Graphics>,
    pub event_proxy: EventLoopProxy<T>
    // pub canvas: Canvas,
}

impl<T> MainWindow<T> {

    pub fn swap_buffers(&self) {
        self.windowed_context.as_ref().unwrap().swap_buffers().expect("swap buffer");
    }

    pub fn handle_redraw_event(&self) {
        self.on_paint(&*self.graphics);
        self.swap_buffers();
    }

    pub fn on_paint(&self, graphics: &dyn Graphics){

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
}

impl<UserEvent> WindowEventHandler<UserEvent> for MainWindow<UserEvent>{
	fn on_mouse_press_event(&self, data: &MouseData){
        println!("on_mouse_press_event");
    }

	fn on_mouse_release_event(&self, data: &MouseData){
        println!("on_mouse_release_event");
    }

	fn on_mouse_move_event(&self, data: &MouseData){
        println!("on_mouse_move_event");
        self.windowed_context.as_ref().map(|f|{
            f.window().request_redraw();
        });
    }

	fn on_keyboard_event(&self, data: &KeyInputData){
    }

    fn on_user_event(&self, data: &UserEvent){
        println!("on_user_event");
    }

}