use crate::app::graphics::Graphics;
use crate::support::{self, Gl};

use glm::{length, vec3, Matrix3};

use glutin::{window::Window, ContextWrapper, PossiblyCurrent};
use windows::Win32::UI::WindowsAndMessaging::ARW_BOTTOMLEFT;

use std::{cell::RefCell, ops::Mul};

pub struct GraphicsOpenGLImpl<'a> {
    pub render_api: RefCell<Gl>,
    pub desktop_size: (u32, u32),
    pub gl_context: &'a ContextWrapper<PossiblyCurrent, Window>,
}

impl<'a> GraphicsOpenGLImpl<'a> {
    pub fn new(desktop_size: (u32, u32), gl_context: &glutin::Context<PossiblyCurrent>) -> Self {
        // init shaders
        let render_api = support::load(gl_context);
        GraphicsOpenGLImpl { render_api: RefCell::new(render_api), desktop_size, gl_context }
    }
}

///
/// returns the transform from screen coordinate to OpenGL NDC coordinate
fn screen_to_ndc_mat(width: u32, height: u32) -> glm::Matrix3<f32> {
    let array = &[
        vec3(2.0 / width as f32, 0.0, -1.0),
        vec3(0.0, -2.0 / height as f32, 1.0),
        vec3(0.0, 0.0, 1.0),
    ];
    let screen_to_ndc = glm::transpose(glm::Matrix3::from_array(array));
    screen_to_ndc
}

#[cfg(test)]
mod opengl_backend_test {

    #[test]
    fn test_ndc_transform_test() {
        let desktop = (1920 as u32, 1080 as u32);
        let transform = super::screen_to_ndc_mat(desktop.0, desktop.1);
        let topleft = glm::vec3(0.0, 0.0, 1.0);
        let topleft_ndc = glm::vec3(-1.0, 1.0, 1.0);
        println!("{:?}", transform * topleft);
    }
}

impl<'a> Graphics for GraphicsOpenGLImpl<'a> {
    #[inline(always)]
    fn draw_rect(&self, x: i32, y: i32, w: u32, h: u32) {
        // Calc transform
        //
        let (x, y, w, h) = (x as f32, y as f32, w as f32, h as f32);
        let mat = screen_to_ndc_mat(self.desktop_size.0, self.desktop_size.1);

        let topleft = vec3(x, y, 1.0);
        let topright = vec3(x + w, y, 1.0);
        let bottomleft = vec3(x, y + h, 1.0);
        let bottomright = vec3(x + w, y + h, 1.0);

        let topleft = mat * topleft;
        let topright = mat * topright;
        let bottomleft = mat * bottomleft;
        let bottomright = mat * bottomright;

        //self.render_api.borrow().update_uniform_mat3(mat);
        self.render_api.borrow().draw_rect_vertex(&[
            topleft.x,
            topleft.y,
            0.1,
            0.1,
            0.3,
            topright.x,
            topright.y,
            0.1,
            0.1,
            0.3,
            bottomleft.x,
            bottomleft.y,
            0.1,
            0.1,
            0.3,
            bottomright.x,
            bottomright.y,
            0.1,
            0.1,
            0.3,
        ]);
    }

    fn draw_rect_frame(&self, x: i32, y: i32, w: u32, h: u32) {
        unimplemented!();
    }

    fn clear(&self, color: (f32, f32, f32, f32)) {
        self.render_api.borrow().clear([color.0, color.1, color.2, color.3]);
    }

    fn submit(&self) {
        self.gl_context.swap_buffers().unwrap();
    }
}
