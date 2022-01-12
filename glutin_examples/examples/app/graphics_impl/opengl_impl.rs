use glm::{vec3, Matrix3};
use std::collections::HashMap;

use crate::support::Gl;

use crate::app::graphics::Graphics;
pub struct GraphicsOpenGLImpl {
    render_api: Gl,
    desktop_size: (u32, u32)
}

impl GraphicsOpenGLImpl{
    pub fn new(render_api: Gl, desktop_size: (u32, u32)) -> Self {
        GraphicsOpenGLImpl { render_api, desktop_size }
    }
}

impl Graphics for GraphicsOpenGLImpl {

    #[inline(always)]
    fn draw_region(&self, x:i32, y:i32, w:u32,h:u32){
        // Calc transform
        //
        let transform = glm::Matrix3::from_array(&[
            vec3(1.0, 1.0, 1.0),
            vec3(1.0, 1.0, 1.0),
            vec3(1.0, 1.0, 1.0),
        ]);
        self.render_api.draw_rect();
    }
}