use image::RgbaImage;
#[cfg(target_arch = "x86")]
use std::arch::x86::_mm_shuffle_epi8;
use std::mem;
use std::sync::Arc;
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

/////////////////////////////
/// MONITOR REGION CAPTURER
/////////////////////////////

pub struct MonitorRegionCapturer {
    pub monitor: Arc<Monitor>,
    pub capture_region: RECT,
    device_context: HDC,
    bitmap: HBITMAP,
}

pub fn get_full_monitor_capturers() -> Result<Vec<MonitorRegionCapturer>> {
    let monitors = get_all_monitors()?;
    let mut capturers = Vec::new();

    for monitor in monitors {
        let region = monitor.info.rect;
        let capturer = get_monitor_capturer(Arc::new(monitor), region);
        capturers.push(capturer);
    }

    Ok(capturers)
}

pub fn get_monitor_capturer(monitor: Arc<Monitor>, capture_region: RECT) -> MonitorRegionCapturer {
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
    // pub fn capture(&self) -> Result<RgbaImage> {
    pub fn capture(&self, metrics: &mut Option<Metrics>) -> Result<RgbaImage> {
        // todo: try https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_2/nf-dxgi1_2-idxgioutputduplication-acquirenextframe
        unsafe {
            if let Some(metrics) = metrics {
                metrics.begin("blit");
            }
            StretchBlt(
                self.device_context,
                0,
                0,
                self.capture_region.width(),
                self.capture_region.height(),
                self.monitor.device_context,
                self.monitor.info.rect.left() - self.capture_region.left(),
                self.monitor.info.rect.top() - self.capture_region.top(),
                self.capture_region.width(),
                self.capture_region.height(),
                SRCCOPY,
            )
            .ok()?;
            if let Some(metrics) = metrics {
                metrics.end("blit");
            }
        };

        let mut bitmap_info = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: self.capture_region.width(),
                biHeight: -self.capture_region.height(),
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
            vec![0u8; (self.capture_region.width() * self.capture_region.height()) as usize * 4];
        let buf_prt = data.as_ptr() as *mut _;

        if let Some(metrics) = metrics {
            metrics.begin("getdibits");
        }
        let err = unsafe {
            GetDIBits(
                self.device_context,
                self.bitmap,
                0,
                self.capture_region.height() as u32,
                Some(buf_prt),
                &mut bitmap_info,
                DIB_RGB_COLORS,
            ) == 0
        };
        if let Some(metrics) = metrics {
            metrics.end("getdibits");
        }

        if err {
            return Err(windows::core::Error::new(S_FALSE, "No RGBA data returned"));
        }

        let mut bitmap = BITMAP::default();
        let bitmap_ptr = <*mut _>::cast(&mut bitmap);

        if let Some(metrics) = metrics {
            metrics.begin("getobject");
        }
        unsafe {
            // Get the BITMAP from the HBITMAP.
            GetObjectW(
                self.bitmap,
                mem::size_of::<BITMAP>() as i32,
                Some(bitmap_ptr),
            );
        }
        if let Some(metrics) = metrics {
            metrics.end("getobject");
        }

        if let Some(metrics) = metrics {
            metrics.begin("shuffle");
        }
        bgra_to_rgba(data.as_mut_slice());
        if let Some(metrics) = metrics {
            metrics.end("shuffle");
        }

        let data = RgbaImage::from_vec(
            self.capture_region.width() as u32,
            self.capture_region.height() as u32,
            data,
        );
        data.ok_or_else(|| windows::core::Error::new(S_FALSE, "Invalid image data"))
    }
}
