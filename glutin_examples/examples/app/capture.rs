use std::alloc::GlobalAlloc;

use super::canvas::Bound2;
use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage, RgbaImage};
// use log::{info, warn};

use windows::{
    core::*,
    Data::Xml::Dom::*,
    Win32::System::{DataExchange::*, Threading::*},
    Win32::{Foundation::*, Graphics::Gdi::*, System::Threading::*, UI::WindowsAndMessaging::*},
};

pub struct CaptureDevice {}

impl CaptureDevice {
    pub fn new() -> Self {
        unsafe { CaptureDevice {} }
    }

    ///
    /// This is the capture routine on windows using GDI
    pub fn capture_image(&self, rect: Bound2) -> RgbaImage {
        let rect = rect.rect();
        unsafe {
            let hdc_src = GetDC(HWND(0)); // the whole desktop
            let hdc_mem = CreateCompatibleDC(hdc_src);
            let bitmap = CreateCompatibleBitmap(hdc_src, rect.2 as i32, rect.3 as i32);
            let old_bitmap = SelectObject(hdc_mem, bitmap);
            BitBlt(
                hdc_mem,
                0,
                0,
                rect.2 as i32,
                rect.3 as i32,
                hdc_src,
                rect.0 as i32,
                rect.1 as i32,
                SRCCOPY,
            );
            let bitmap = SelectObject(hdc_mem, old_bitmap);

            let mut bitmap_info: BITMAP = BITMAP::default();
            use std::ffi::c_void;
            use std::mem;
            GetObjectW(
                bitmap,
                mem::size_of::<BITMAP>() as i32,
                (&mut bitmap_info) as *mut _ as *mut c_void,
            );
            // println!("bitmap info: {:?}", bitmap_info);

            let mut bi: BITMAPINFOHEADER = BITMAPINFOHEADER::default();

            bi.biSize = mem::size_of::<BITMAPINFOHEADER>() as u32;
            bi.biWidth = bitmap_info.bmWidth;
            bi.biHeight = bitmap_info.bmHeight;
            bi.biPlanes = 1;
            bi.biBitCount = 32;
            bi.biCompression = BI_RGB as u32;
            bi.biSizeImage = 0;
            bi.biXPelsPerMeter = 0;
            bi.biYPelsPerMeter = 0;
            bi.biClrUsed = 0;
            bi.biClrImportant = 0;

            let dw_bmp_size =
                ((bitmap_info.bmWidth * bi.biBitCount as i32 + 31) / 32) * 4 * bitmap_info.bmHeight;

            let mut raw_data = vec![0u8; dw_bmp_size as usize];

            GetDIBits(
                hdc_mem,
                HBITMAP(bitmap.0),
                0,
                bitmap_info.bmHeight as u32,
                raw_data.as_mut_ptr() as *mut c_void,
                &mut bi as *mut _ as *mut BITMAPINFO,
                DIB_RGB_COLORS,
            );

            ReleaseDC(HWND(0), hdc_src);
            DeleteDC(hdc_mem);

            ImageBuffer::from_fn(bitmap_info.bmWidth as u32, bitmap_info.bmHeight as u32, |x, y| {
                let ind = (bitmap_info.bmHeight as u32 - 1u32 - y) * bitmap_info.bmWidth as u32 + x;
                let ind = ind as usize;
                let ptr = raw_data.as_ptr().add(ind * 4);
                image::Rgba([*ptr.add(2), *ptr.add(1), *ptr, *ptr.add(3)])
            })
        }
    }
}

impl Drop for CaptureDevice {
    fn drop(&mut self) {}
}
