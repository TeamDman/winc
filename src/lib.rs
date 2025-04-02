mod metrics;
mod monitor;
mod monitor_info;
mod monitor_region_capturer;
mod rect_extensions;
mod shuffle;
mod tests;
mod direct;

pub mod prelude {
    pub use crate::metrics::*;
    pub use crate::monitor::*;
    pub use crate::monitor_info::*;
    pub use crate::monitor_region_capturer::*;
    pub use crate::rect_extensions::*;
    pub use crate::shuffle::*;
    pub use windows::Win32::Foundation::RECT;
}
