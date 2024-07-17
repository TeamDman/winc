
mod monitor;
mod monitor_region_capturer;
mod monitor_info;
mod tests;
mod shuffle;
mod rect_extensions;
mod metrics;

pub mod prelude {
    pub use crate::monitor_info::*;
    pub use crate::monitor::*;
    pub use crate::monitor_region_capturer::*;
    pub use crate::shuffle::*;
    pub use crate::rect_extensions::*;
    pub use crate::metrics::*;
    pub use windows::Win32::Foundation::RECT;
}