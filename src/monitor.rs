use std::ptr;
use widestring::U16CString;
use windows::core::Result;
use windows::core::PCWSTR;
use windows::Win32::Foundation::E_INVALIDARG;
use windows::Win32::Graphics::Gdi::CreateDCW;
use windows::Win32::Graphics::Gdi::HDC;

use crate::prelude::get_monitor_infos;
use crate::prelude::MonitorInfo;

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
