use super::canvas::Bound2;
use image::{
    DynamicImage::ImageRgba8, GenericImage, GenericImageView, ImageBuffer, RgbImage, Rgba,
    RgbaImage,
    codecs::gif::GifEncoder as Encoder,
};

use crate::platform::windows::capture_impl::CaptureImplWin;

use std::future::Future;
use std::sync::Arc;
use std::sync::Mutex;
// TODO:: remove platform-specified mods

use windows::{
    core::*,
    Data::Xml::Dom::*,
    Win32::System::{DataExchange::*, Threading::*},
    Win32::{Foundation::*, Graphics::Gdi::*, System::Threading::*, UI::WindowsAndMessaging::*},
};

pub struct CaptureDevice {
    pub runtime: tokio::runtime::Runtime,
}

impl CaptureDevice {
    pub fn new() -> std::io::Result<Self> {
        Ok(CaptureDevice { runtime: tokio::runtime::Runtime::new()? })
    }

    pub fn capture_image(&self, rect: Bound2) -> RgbaImage {
        CaptureImplWin::new(HWND(0), rect).capture_image()
    }

    pub fn capture_gif_async(
        &self,
        rect: Bound2,
        fps: u32,
        duration: f64,
        finished_cb: Box<dyn FnOnce(Vec<RgbaImage>) + Send>,
        buf_write_finish_cb: impl FnOnce(Arc<Mutex<Encoder<Vec<u8>>>>) + Send + 'static + Copy,
    ) {
        if fps <= 0 || fps > 60 {
            println!("Wrong fps: {}, it should be in range (0, 60]", fps);
        }
        use std::time::Instant;
        use tokio::time::Interval;
        let interval = 1.0 / fps as f64;
        let dur = std::time::Duration::from_secs_f64(duration);
        let width = rect.get_width();
        let height = rect.get_height();

        self.runtime.spawn(async move {
            //let mut task = tokio::time::interval(std::time::Duration::from_secs_f64(interval));
            let interval = std::time::Duration::from_secs_f64(interval);
            let mut cap_impl = Arc::new(Mutex::new(CaptureImplWin::new(HWND(0), rect)));
            let total_frames = fps * duration as u32;
            let mut frames = 0;
            let mut elapse = Instant::now();
            let end = elapse + dur;
            //let mut frames_data = vec![];
            let mut encode_buf = vec![0u8; 0];
            let encoder = Arc::new(Mutex::new(Encoder::new(encode_buf)));
            while frames < total_frames && elapse < end {
                let img = async {
                    let mut cap = cap_impl.lock().unwrap();
                    cap.capture_image()
                }
                .await;
                println!("capture image");
                let encoder_copy = encoder.clone();
                elapse += interval;
                frames += 1;
                tokio::spawn(async move {
                    let mut frame = image::Frame::new(img);
                    encoder_copy.lock().unwrap().encode_frame(frame).unwrap();
                    println!("encode");
                    if frames == total_frames{
                        buf_write_finish_cb(encoder_copy.clone());
                    }
                });
            }
            //finished_cb(frames_data);
        });
    }
}
