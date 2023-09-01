#[cfg(windows)]
extern crate winapi;

#[cfg(windows)]
pub use winapi::um::commctrl::{
    TDCBF_CANCEL_BUTTON, TDCBF_CLOSE_BUTTON, TDCBF_NO_BUTTON, TDCBF_OK_BUTTON, TDCBF_RETRY_BUTTON,
    TDCBF_YES_BUTTON, TDF_ALLOW_DIALOG_CANCELLATION, TDF_CAN_BE_MINIMIZED, TDF_ENABLE_HYPERLINKS,
    TDF_EXPANDED_BY_DEFAULT, TDF_EXPAND_FOOTER_AREA, TDF_NO_DEFAULT_RADIO_BUTTON,
    TDF_NO_SET_FOREGROUND, TDF_POSITION_RELATIVE_TO_WINDOW, TDF_RTL_LAYOUT,
    TDF_SHOW_MARQUEE_PROGRESS_BAR, TDF_SHOW_PROGRESS_BAR, TDF_SIZE_TO_CONTENT,
    TDF_USE_COMMAND_LINKS, TDF_USE_COMMAND_LINKS_NO_ICON, TDF_VERIFICATION_FLAG_CHECKED,
    TDM_SET_PROGRESS_BAR_MARQUEE, TDM_SET_PROGRESS_BAR_POS, TDN_BUTTON_CLICKED, TDN_CREATED,
    TDN_DESTROYED, TDN_HYPERLINK_CLICKED, TDN_NAVIGATED, TD_ERROR_ICON, TD_INFORMATION_ICON,
    TD_SHIELD_ICON, TD_WARNING_ICON,
};

#[cfg(not(windows))]
pub const TDCBF_CANCEL_BUTTON: u32 = 0;
#[cfg(not(windows))]
pub const TDCBF_CLOSE_BUTTON: u32 = 0;
#[cfg(not(windows))]
pub const TDCBF_NO_BUTTON: u32 = 0;
#[cfg(not(windows))]
pub const TDCBF_OK_BUTTON: u32 = 0;
#[cfg(not(windows))]
pub const TDCBF_RETRY_BUTTON: u32 = 0;
#[cfg(not(windows))]
pub const TDCBF_YES_BUTTON: u32 = 0;
#[cfg(not(windows))]
pub const TDF_ALLOW_DIALOG_CANCELLATION: u32 = 0;
#[cfg(not(windows))]
pub const TDF_CAN_BE_MINIMIZED: u32 = 0;
#[cfg(not(windows))]
pub const TDF_ENABLE_HYPERLINKS: u32 = 0;
#[cfg(not(windows))]
pub const TDF_EXPANDED_BY_DEFAULT: u32 = 0;
#[cfg(not(windows))]
pub const TDF_EXPAND_FOOTER_AREA: u32 = 0;
#[cfg(not(windows))]
pub const TDF_NO_DEFAULT_RADIO_BUTTON: u32 = 0;
#[cfg(not(windows))]
pub const TDF_NO_SET_FOREGROUND: u32 = 0;
#[cfg(not(windows))]
pub const TDF_POSITION_RELATIVE_TO_WINDOW: u32 = 0;
#[cfg(not(windows))]
pub const TDF_RTL_LAYOUT: u32 = 0;
#[cfg(not(windows))]
pub const TDF_SIZE_TO_CONTENT: u32 = 0;
#[cfg(not(windows))]
pub const TDF_USE_COMMAND_LINKS: u32 = 0;
#[cfg(not(windows))]
pub const TDF_USE_COMMAND_LINKS_NO_ICON: u32 = 0;
#[cfg(not(windows))]
pub const TDF_VERIFICATION_FLAG_CHECKED: u32 = 0;
#[cfg(not(windows))]
pub const TD_ERROR_ICON: *mut u16 = std::ptr::null_mut();
#[cfg(not(windows))]
pub const TD_INFORMATION_ICON: *mut u16 = std::ptr::null_mut();
#[cfg(not(windows))]
pub const TD_SHIELD_ICON: *mut u16 = std::ptr::null_mut();
#[cfg(not(windows))]
pub const TD_WARNING_ICON: *mut u16 = std::ptr::null_mut();
#[cfg(not(windows))]
pub const TDF_SHOW_MARQUEE_PROGRESS_BAR: u32 = 0;
#[cfg(not(windows))]
pub const TDF_SHOW_PROGRESS_BAR: u32 = 0;
#[cfg(not(windows))]
pub const TDM_SET_PROGRESS_BAR_MARQUEE: u32 = 0;
#[cfg(not(windows))]
pub const TDM_SET_PROGRESS_BAR_POS: u32 = 0;
#[cfg(not(windows))]
pub const TDN_BUTTON_CLICKED: u32 = 0;
#[cfg(not(windows))]
pub const TDN_CREATED: u32 = 0;
#[cfg(not(windows))]
pub const TDN_DESTROYED: u32 = 0;
#[cfg(not(windows))]
pub const TDN_HYPERLINK_CLICKED: u32 = 0;
#[cfg(not(windows))]
pub const TDN_NAVIGATED: u32 = 0;
