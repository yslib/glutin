#![allow(unused)]
mod app;
mod support;
mod misc;

use app::{
    application::{Application, ApplicationBuilder},
    event::{UserEvent}
};
use glutin::event_loop::EventLoop;

fn main() {
    let event_loop = EventLoop::<UserEvent>::with_user_event();
    ApplicationBuilder::<UserEvent>::new()
        .with_name("EasyCapture")
        .build(&event_loop)
        .expect("failed to create application")
        .run(event_loop);
}