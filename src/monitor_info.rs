

#[cfg(target_arch = "x86")]
use std::arch::x86::_mm_shuffle_epi8;
use std::mem;
use widestring::U16CString;
use windows::core::Result;
use windows::Win32::Foundation::BOOL;
use windows::Win32::Foundation::LPARAM;
use windows::Win32::Foundation::RECT;
use windows::Win32::Graphics::Gdi::EnumDisplayMonitors;
use windows::Win32::Graphics::Gdi::GetMonitorInfoW;
use windows::Win32::Graphics::Gdi::HDC;
use windows::Win32::Graphics::Gdi::HMONITOR;
use windows::Win32::Graphics::Gdi::MONITORINFOEXW;

pub type MonitorId = u32;

#[derive(Debug)]
pub struct MonitorInfo {
    pub id: MonitorId,
    pub name: String,
    pub rect: RECT,
    pub work_area: RECT, // the area of the monitor not covered by the taskbar
    pub is_primary: bool,
}

pub fn get_monitor_infos() -> Result<Vec<MonitorInfo>> {
    // box it up so we can pass it to the callback
    let results: *mut Vec<MONITORINFOEXW> = Box::into_raw(Box::default());

    // use proc method to iterate monitors and collect into results vec
    unsafe {
        EnumDisplayMonitors(
            HDC::default(),
            None,
            Some(monitor_enum_proc),
            LPARAM(results as isize),
        )
        .ok()?;
    };

    // convert results back into a vec
    let results = unsafe { &Box::from_raw(results) };

    // convert vec of MONITORINFOEXW into vec of MonitorInfo
    let results = results
        .iter()
        .map(|info| {
            let sz_device_ptr = info.szDevice.as_ptr();
            let sz_device_string =
                unsafe { U16CString::from_ptr_str(sz_device_ptr).to_string_lossy() };
            MonitorInfo {
                id: fxhash::hash32(sz_device_string.as_bytes()), // same algorithm as screen crate
                name: sz_device_string,
                rect: info.monitorInfo.rcMonitor,
                work_area: info.monitorInfo.rcWork,
                is_primary: info.monitorInfo.dwFlags == 1,
            }
        })
        .collect::<Vec<MonitorInfo>>();
    Ok(results)
}

extern "system" fn monitor_enum_proc(
    h_monitor: HMONITOR,
    _: HDC,
    _: *mut RECT,
    data: LPARAM,
) -> BOOL {
    let results = unsafe { Box::from_raw(data.0 as *mut Vec<MONITORINFOEXW>) };
    let results = Box::leak(results);

    match get_monitor_info_exw(h_monitor) {
        Ok(monitor_info_exw) => {
            results.push(monitor_info_exw);
            BOOL::from(true)
        }
        Err(_) => BOOL::from(false),
    }
}

fn get_monitor_info_exw(h_monitor: HMONITOR) -> Result<MONITORINFOEXW> {
    let mut monitor_info_exw: MONITORINFOEXW = unsafe { mem::zeroed() };
    monitor_info_exw.monitorInfo.cbSize = mem::size_of::<MONITORINFOEXW>() as u32;
    let monitor_info_exw_ptr = <*mut _>::cast(&mut monitor_info_exw);

    unsafe {
        GetMonitorInfoW(h_monitor, monitor_info_exw_ptr).ok()?;
    };
    Ok(monitor_info_exw)
}
