use glm::{vec2};
use std::rc::Rc;
use crate::Graphics;

use super::graphics_impl::opengl_impl::GraphicsOpenGLImpl;

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

pub trait Renderable{
	fn update(&self, graphics:&Graphics);
}

pub struct RegionSelector{
	bound:Bound2,
	visible:bool
}

impl Renderable for RegionSelector{
	fn update(&self, graphics:&Graphics) {
		let rect = self.bound.rect();
        graphics.draw_rect(rect.0, rect.1, rect.2, rect.3);
	}
}

pub struct Canvas{
	objects: Vec<Rc<dyn Renderable>>,
	graphics: Box<dyn Graphics>
}

impl Canvas{
	pub fn add_object(&mut self, object:Rc<dyn Renderable>){
		self.objects.push(object);
	}

	pub fn on_draw(&self){
		self.graphics.clear((0.0,0.0,0.0,0.0));
		for obj in &self.objects{
			obj.update(&(*self.graphics));
		}
	}
}