use crate::app::graphics::Graphics;
use crate::support;
use crate::support::Gl;
use glm::{vec3, Matrix3, length};

use glutin::PossiblyCurrent;

use std::{cell::RefCell, ops::Mul};

pub struct GraphicsOpenGLImpl {
    pub render_api: RefCell<Gl>,
    pub desktop_size: (u32, u32),
}

impl GraphicsOpenGLImpl {
    pub fn new(desktop_size: (u32, u32), gl_context: &glutin::Context<PossiblyCurrent>) -> Self {
        // init shaders
        let render_api = support::load(gl_context);
        GraphicsOpenGLImpl { render_api:RefCell::new(render_api), desktop_size }
    }
}

impl Graphics for GraphicsOpenGLImpl {
    #[inline(always)]
    fn draw_rect(&self, x: i32, y: i32, w: u32, h: u32) {
        // Calc transform
		//
        let (x, y, w, h)= (x as f32, y as f32, w as f32, h as f32);
        let screen_center = vec3((2.0 * x + w) / 2.0, (2.0 * y + h) /2.0, 1.0);
        let scalex = w as f32 / self.desktop_size.0 as f32;
        let scaley = h as f32 / self.desktop_size.1 as f32;

        let array = &[
			vec3(2.0/self.desktop_size.0 as f32, 0.0,-1.0),
			vec3(0.0, - 2.0 / self.desktop_size.1 as f32, 1.0),
			vec3(0.0,0.0,1.0)
        ];

        let screen_to_ndc = glm::transpose(glm::Matrix3::from_array(array));

        let ndc_center = screen_to_ndc *screen_center;

        let array = &[
            vec3(1.0, 0.0, ndc_center.x),
            vec3(0.0,1.0,ndc_center.y),
            vec3(0.0,0.0,1.0),
        ];
        let ndc_translate = glm::transpose(glm::Matrix3::from_array(array));

        let array = &[
            vec3(scalex, 0.0, 0.0),
            vec3(0.0,scaley,0.0),
            vec3(0.0,0.0,1.0),
        ];
        let ndc_scale = glm::transpose(glm::Matrix3::from_array(array));

        let mat =((ndc_scale)*(ndc_translate)*(screen_to_ndc));

        println!("{:?}", mat);

        let mat = glm::inverse(&mat);

        //self.render_api.borrow().update_uniform_mat3(mat);
        //self.render_api.borrow().draw_frame();
        self.render_api.borrow().draw_rect_vertex();
    }

    fn draw_rect_frame(&self, x: i32, y: i32, w: u32, h: u32) {
		unimplemented!();
	}

    fn clear(&self, color: (f32, f32, f32, f32)) {
        self.render_api.borrow().clear([color.0, color.1, color.2, color.3]);
    }
}
