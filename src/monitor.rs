use image::RgbaImage;
#[cfg(target_arch = "x86")]
use std::arch::x86::_mm_shuffle_epi8;
use std::arch::x86_64::__m128i;
use std::arch::x86_64::_mm_loadu_si128;
use std::arch::x86_64::_mm_setr_epi8;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::_mm_shuffle_epi8;
use std::arch::x86_64::_mm_storeu_si128;
use std::mem;
use std::ops::Deref;
use std::ptr;
use std::rc::Rc;
use std::sync::Arc;
use widestring::U16CString;
use windows::core::Result;
use windows::core::PCWSTR;
use windows::Win32::Foundation::BOOL;
use windows::Win32::Foundation::E_INVALIDARG;
use windows::Win32::Foundation::LPARAM;
pub use windows::Win32::Foundation::RECT;
use windows::Win32::Graphics::Gdi::BitBlt;
use windows::Win32::Graphics::Gdi::CreateCompatibleBitmap;
use windows::Win32::Graphics::Gdi::CreateCompatibleDC;
use windows::Win32::Graphics::Gdi::CreateDCW;
use windows::Win32::Graphics::Gdi::DeleteDC;
use windows::Win32::Graphics::Gdi::DeleteObject;
use windows::Win32::Graphics::Gdi::EnumDisplayMonitors;
use windows::Win32::Graphics::Gdi::GetDIBits;
use windows::Win32::Graphics::Gdi::GetMonitorInfoW;
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
use windows::Win32::Graphics::Gdi::HMONITOR;
use windows::Win32::Graphics::Gdi::MONITORINFOEXW;
use windows::Win32::Graphics::Gdi::RGBQUAD;
use windows::Win32::Graphics::Gdi::SRCCOPY;
use windows::Win32::Graphics::Gdi::STRETCH_HALFTONE;

use crate::prelude::get_monitor_infos;
use crate::prelude::MonitorInfo;

//////////////////
/// GET MONITORS
//////////////////
pub struct Monitor {
    pub info: MonitorInfo,
    pub device_context: HDC,
}

pub fn get_all_monitors() -> Result<Vec<Monitor>> {
    let monitor_infos = get_monitor_infos()?;
    let mut monitors = Vec::new();

    for monitor_info in monitor_infos {
        // intermediate variables are required to ensure the pointer contents remain in scope
        let a = U16CString::from_str(&monitor_info.name).map_err(|e| {
            windows::core::Error::new(
                E_INVALIDARG,
                format!("Invalid null character at index {}", e.nul_position()),
            )
        })?;
        let b = a.as_ptr();
        let name_pcwstr = PCWSTR(b);
        let device_context =
            unsafe { CreateDCW(name_pcwstr, name_pcwstr, PCWSTR(ptr::null()), None) };

        monitors.push(Monitor {
            info: monitor_info,
            device_context,
        });
    }

    Ok(monitors)
}
