use super::canvas::Bound2;
use image::{
    DynamicImage::ImageRgba8, GenericImage, GenericImageView, ImageBuffer, RgbImage, RgbaImage, Rgba,
};

use crate::platform::windows::capture_impl::CaptureImplWin;

// TODO:: remove platform-specified mods

use windows::{
    core::*,
    Data::Xml::Dom::*,
    Win32::System::{DataExchange::*, Threading::*},
    Win32::{Foundation::*, Graphics::Gdi::*, System::Threading::*, UI::WindowsAndMessaging::*},
};

pub struct CaptureDevice{
    pub runtime: tokio::runtime::Runtime
}

impl CaptureDevice {
    pub fn new() -> Self {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        CaptureDevice{runtime}
    }

    pub fn capture_image(&self, rect: Bound2) -> RgbaImage {
        CaptureImplWin::new(HWND(0), rect).capture_image()
    }

    pub fn capture_image_async(&self, rect: Bound2, fps:u32, duration:f64, finished_cb: Box<dyn FnOnce(Vec<Vec<u8>>)+ Send>){
        if fps <= 0 || fps > 60{
            println!("Wrong fps: {}, it should be in range (0, 60]", fps);
        }
        use tokio::time::Interval;
        use std::time::Instant;
        let interval = 1.0 / fps as f64;
        let dur = std::time::Duration::from_secs_f64(duration);

        self.runtime.spawn(async move{
            let mut task = tokio::time::interval(std::time::Duration::from_secs_f64(interval));
            let mut cap = CaptureImplWin::new(HWND(0), rect);
            let total_frames = fps * duration as u32;
            let mut frames = 0;
            let mut elapse = Instant::now();
            let end = elapse + dur;
            let mut frames_data = vec![];
            while frames < total_frames {
                cap.capture_image_raw();
                frames_data.push(cap.raw_data.clone());
                elapse += task.period();
                frames += 1;
            }
            finished_cb(frames_data);
        });
    }
}
