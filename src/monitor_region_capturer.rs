use image::RgbaImage;
#[cfg(target_arch = "x86")]
use std::arch::x86::_mm_shuffle_epi8;
use std::mem;
use std::rc::Rc;
use windows::core::Result;
use windows::Win32::Foundation::RECT;
use windows::Win32::Foundation::S_FALSE;
use windows::Win32::Graphics::Gdi::CreateCompatibleBitmap;
use windows::Win32::Graphics::Gdi::CreateCompatibleDC;
use windows::Win32::Graphics::Gdi::DeleteDC;
use windows::Win32::Graphics::Gdi::DeleteObject;
use windows::Win32::Graphics::Gdi::GetDIBits;
use windows::Win32::Graphics::Gdi::GetObjectW;
use windows::Win32::Graphics::Gdi::SelectObject;
use windows::Win32::Graphics::Gdi::SetStretchBltMode;
use windows::Win32::Graphics::Gdi::StretchBlt;
use windows::Win32::Graphics::Gdi::BITMAP;
use windows::Win32::Graphics::Gdi::BITMAPINFO;
use windows::Win32::Graphics::Gdi::BITMAPINFOHEADER;
use windows::Win32::Graphics::Gdi::DIB_RGB_COLORS;
use windows::Win32::Graphics::Gdi::HBITMAP;
use windows::Win32::Graphics::Gdi::HDC;
use windows::Win32::Graphics::Gdi::RGBQUAD;
use windows::Win32::Graphics::Gdi::SRCCOPY;
use windows::Win32::Graphics::Gdi::STRETCH_HALFTONE;

use crate::prelude::bgra_to_rgba;
use crate::prelude::get_all_monitors;
use crate::prelude::HasHeight;
use crate::prelude::HasLeft;
use crate::prelude::HasTop;
use crate::prelude::HasWidth;
use crate::prelude::Metrics;
use crate::prelude::Monitor;

pub struct MonitorRegionCapturer {
    pub monitor: Rc<Monitor>,
    pub capture_region: RECT,
    device_context: HDC,
    bitmap: HBITMAP,
}

pub fn get_full_monitor_capturers() -> Result<Vec<MonitorRegionCapturer>> {
    let monitors = get_all_monitors()?;
    let mut capturers = Vec::new();

    for monitor in monitors {
        let region = monitor.info.rect;
        let capturer = get_monitor_capturer(Rc::new(monitor), region);
        capturers.push(capturer);
    }

    Ok(capturers)
}

pub fn get_monitor_capturer(monitor: Rc<Monitor>, capture_region: RECT) -> MonitorRegionCapturer {
    let capture_device_context = unsafe { CreateCompatibleDC(monitor.device_context) };
    let bitmap = unsafe {
        CreateCompatibleBitmap(
            monitor.device_context,
            capture_region.width(),
            capture_region.height(),
        )
    };

    unsafe {
        SelectObject(capture_device_context, bitmap);
        SetStretchBltMode(monitor.device_context, STRETCH_HALFTONE);
    };

    MonitorRegionCapturer {
        monitor,
        device_context: capture_device_context,
        bitmap,
        capture_region,
    }
}

impl Drop for MonitorRegionCapturer {
    fn drop(&mut self) {
        unsafe {
            if let Err(e) = DeleteObject(self.bitmap).ok() {
                eprintln!("winc error deleting bitmap: {:?}", e);
            };
            if let Err(e) = DeleteDC(self.device_context).ok() {
                eprintln!("winc error deleting device context: {:?}", e);
            };
        }
    }
}
impl MonitorRegionCapturer {
    pub fn capture(&self, metrics: &mut Metrics) -> Result<RgbaImage> {
        let capture_region_width = self.capture_region.width();
        let capture_region_height = self.capture_region.height();
        // todo: try https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_2/nf-dxgi1_2-idxgioutputduplication-acquirenextframe
        unsafe {
            metrics.begin("blit");
            StretchBlt(
                self.device_context,
                0,
                0,
                capture_region_width,
                capture_region_height,
                self.monitor.device_context,
                self.monitor.info.rect.left() - self.capture_region.left(),
                self.monitor.info.rect.top() - self.capture_region.top(),
                capture_region_width,
                capture_region_height,
                SRCCOPY,
            )
            .ok()?;
            metrics.end("blit");
        };

        let mut bitmap_info = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: capture_region_width,
                biHeight: -capture_region_height,
                biPlanes: 1,
                biBitCount: 32,
                biCompression: 0,
                biSizeImage: 0,
                biXPelsPerMeter: 0,
                biYPelsPerMeter: 0,
                biClrUsed: 0,
                biClrImportant: 0,
            },
            bmiColors: [RGBQUAD::default(); 1],
        };

        let mut data =
            vec![0u8; (capture_region_width * capture_region_height) as usize * 4];
        let buf_prt = data.as_ptr() as *mut _;

        metrics.begin("getdibits");
        let err = unsafe {
            GetDIBits(
                self.device_context,
                self.bitmap,
                0,
                capture_region_height as u32,
                Some(buf_prt),
                &mut bitmap_info,
                DIB_RGB_COLORS,
            ) == 0
        };
        metrics.end("getdibits");

        if err {
            return Err(windows::core::Error::new(S_FALSE, "No RGBA data returned"));
        }

        let mut bitmap = BITMAP::default();
        let bitmap_ptr = <*mut _>::cast(&mut bitmap);

        metrics.begin("getobject");
        unsafe {
            // Get the BITMAP from the HBITMAP.
            GetObjectW(
                self.bitmap,
                mem::size_of::<BITMAP>() as i32,
                Some(bitmap_ptr),
            );
        }
        metrics.end("getobject");

        metrics.begin("shuffle");
        bgra_to_rgba(data.as_mut_slice());
        metrics.end("shuffle");

        metrics.begin("image");
        let data = RgbaImage::from_vec(
            capture_region_width as u32,
            capture_region_height as u32,
            data,
        );
        metrics.end("image");
        data.ok_or_else(|| windows::core::Error::new(S_FALSE, "Invalid image data"))
    }
}
