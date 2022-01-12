#![allow(unused)]

mod app;
use app::application::{Application, ApplicationBuilder};

mod window_system;
use window_system::{glutin::GlutinSystem, WindowSystem};

mod misc;
mod support;


fn main() {
    let window_system = GlutinSystem::new();
    ApplicationBuilder::new()
    .with_window_system(window_system)
    .with_name("shortcut".to_owned())
    .exec()
}