use super::{canvas::Bound2, image::Image};
use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};

use windows::{
    core::*,
    Data::Xml::Dom::*,
    Win32::System::{DataExchange::*, Threading::*},
    Win32::{Foundation::*, Graphics::Gdi::*, System::Threading::*, UI::WindowsAndMessaging::*},
};

pub struct CaptureDevice {
    hdc: HDC,
    hdc_temp: CreatedHDC,
}

impl CaptureDevice {
    pub fn new() -> Self {
        unsafe {
            let hdc = GetDC(HWND(0));
            let hdc_temp = CreateCompatibleDC(hdc);
            CaptureDevice { hdc, hdc_temp }
        }
    }

    pub fn capture_image(&self, rect: Bound2) {
        let rect = rect.rect();
        unsafe {
            let bitmap = CreateCompatibleBitmap(self.hdc, rect.2 as i32, rect.3 as i32);
            BitBlt(
                self.hdc_temp,
                0,
                0,
                rect.2 as i32,
                rect.3 as i32,
                self.hdc,
                rect.0 as i32,
                rect.1 as i32,
                SRCCOPY,
            );
            SelectObject(self.hdc_temp, bitmap);
        }
        println!("capture {:?}", rect);
    }
}

impl Drop for CaptureDevice {
    fn drop(&mut self) {}
}
