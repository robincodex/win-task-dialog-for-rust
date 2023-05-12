#[cfg(windows)]
extern crate winapi;

#[cfg(windows)]
use widestring::U16CString;
#[cfg(windows)]
use winapi::shared::basetsd::LONG_PTR;
#[cfg(windows)]
use winapi::shared::minwindef::*;
#[cfg(windows)]
use winapi::shared::windef::HWND;
#[cfg(windows)]
use winapi::um::commctrl::{
    TASKDIALOGCONFIG_u1, TASKDIALOGCONFIG_u2, TaskDialogIndirect, HRESULT, TASKDIALOGCONFIG,
    TASKDIALOG_BUTTON, TASKDIALOG_COMMON_BUTTON_FLAGS, TASKDIALOG_FLAGS,
};
#[cfg(windows)]
use winapi::um::commctrl::{
    TDE_CONTENT, TDE_EXPANDED_INFORMATION, TDE_FOOTER, TDE_MAIN_INSTRUCTION,
    TDM_UPDATE_ELEMENT_TEXT,
};
#[cfg(windows)]
use winapi::um::commctrl::{
    TDF_SHOW_MARQUEE_PROGRESS_BAR, TDF_SHOW_PROGRESS_BAR, TDM_SET_PROGRESS_BAR_MARQUEE,
    TDM_SET_PROGRESS_BAR_POS, TDN_CREATED, TDN_DESTROYED, TDN_HYPERLINK_CLICKED,
};
#[cfg(windows)]
use winapi::um::libloaderapi::GetModuleHandleA;
#[cfg(windows)]
use winapi::um::winnt::LPWSTR;
#[cfg(windows)]
use winapi::um::winuser::SendMessageA;

#[cfg(not(windows))]
type HWND = *mut usize;

#[cfg(not(windows))]
type HMODULE = *mut usize;

#[cfg(not(windows))]
type LPWSTR = *mut u16;

#[cfg(not(windows))]
#[allow(non_camel_case_types)]
type TASKDIALOG_FLAGS = u32;

#[cfg(not(windows))]
#[allow(non_camel_case_types)]
type TASKDIALOG_COMMON_BUTTON_FLAGS = u32;

use std::ptr::null_mut;
use std::{io::Error, usize};

mod constants;
pub use constants::*;
pub struct TaskDialogConfig {
    pub parent: HWND,
    pub instance: HMODULE,
    pub flags: TASKDIALOG_FLAGS,
    pub common_buttons: TASKDIALOG_COMMON_BUTTON_FLAGS,
    pub window_title: String,
    pub main_instruction: String,
    pub content: String,
    pub verification_text: String,
    pub expanded_information: String,
    pub expanded_control_text: String,
    pub collapsed_control_text: String,
    pub footer: String,
    pub buttons: Vec<TaskDialogButton>,
    pub default_button: i32,
    pub radio_buttons: Vec<TaskDialogButton>,
    pub default_radio_buttons: i32,
    pub main_icon: LPWSTR,
    pub footer_icon: LPWSTR,
    /** When created dialog, the value set to HWND. */
    pub dialog_hwnd: HWND,
    /** When close the dialog, the value set to true, default is false. */
    pub is_destroyed: bool,
    pub hyperlinkclicked_callback: fn(link: String) -> (),
}

impl Default for TaskDialogConfig {
    fn default() -> Self {
        TaskDialogConfig {
            parent: null_mut(),
            instance: null_mut(),
            flags: 0,
            common_buttons: TDCBF_CANCEL_BUTTON,
            window_title: "".to_string(),
            main_instruction: "".to_string(),
            content: "".to_string(),
            verification_text: "".to_string(),
            expanded_information: "".to_string(),
            expanded_control_text: "".to_string(),
            collapsed_control_text: "".to_string(),
            footer: "".to_string(),
            buttons: vec![],
            default_button: 0,
            radio_buttons: vec![],
            default_radio_buttons: 0,
            main_icon: null_mut(),
            footer_icon: null_mut(),
            dialog_hwnd: null_mut(),
            is_destroyed: false,
            hyperlinkclicked_callback: |_| {},
        }
    }
}

#[cfg(windows)]
impl TaskDialogConfig {
    /**
    Add TDF_SHOW_PROGRESS_BAR flag on marquee is false;

    Add TDF_SHOW_MARQUEE_PROGRESS_BAR flag on marquee is true;

    https://docs.microsoft.com/en-us/windows/win32/controls/progress-bar-control
    */
    pub fn enable_process_bar(&mut self, marquee: bool) {
        if marquee {
            if self.flags & TDF_SHOW_MARQUEE_PROGRESS_BAR != TDF_SHOW_MARQUEE_PROGRESS_BAR {
                self.flags = self.flags | TDF_SHOW_MARQUEE_PROGRESS_BAR;
            }
        } else {
            if self.flags & TDF_SHOW_PROGRESS_BAR != TDF_SHOW_PROGRESS_BAR {
                self.flags = self.flags | TDF_SHOW_PROGRESS_BAR;
            }
        }
    }

    /** Set status or animation time of marquee progress bar */
    pub fn set_process_bar_marquee(&mut self, enable: bool, time: isize) {
        if self.dialog_hwnd == null_mut() {
            return;
        }
        unsafe {
            SendMessageA(
                self.dialog_hwnd,
                TDM_SET_PROGRESS_BAR_MARQUEE,
                if enable {
                    TRUE as usize
                } else {
                    FALSE as usize
                },
                time,
            );
        }
    }

    /** Set the percentage of the progress bar */
    pub fn set_process_bar(&mut self, percentage: usize) {
        if self.dialog_hwnd == null_mut() {
            return;
        }
        unsafe {
            SendMessageA(self.dialog_hwnd, TDM_SET_PROGRESS_BAR_POS, percentage, 0);
        }
    }

    /** Set the content text */
    pub fn set_content(&mut self, content: &str) {
        if self.dialog_hwnd == null_mut() {
            return;
        }
        let content_wchar = U16CString::from_str(content).unwrap();
        unsafe {
            SendMessageA(
                self.dialog_hwnd,
                TDM_UPDATE_ELEMENT_TEXT,
                TDE_CONTENT as _,
                content_wchar.as_ptr() as _,
            );
        }
    }

    /** Set the main instruction text */
    pub fn set_main_instruction(&mut self, main_instruction: &str) {
        if self.dialog_hwnd == null_mut() {
            return;
        }
        let main_instruction_wchar = U16CString::from_str(main_instruction).unwrap();
        unsafe {
            SendMessageA(
                self.dialog_hwnd,
                TDM_UPDATE_ELEMENT_TEXT,
                TDE_MAIN_INSTRUCTION as _,
                main_instruction_wchar.as_ptr() as _,
            );
        }
    }

    /** Set the footer text */
    pub fn set_footer(&mut self, footer: &str) {
        if self.dialog_hwnd == null_mut() {
            return;
        }
        let footer_wchar = U16CString::from_str(footer).unwrap();
        unsafe {
            SendMessageA(
                self.dialog_hwnd,
                TDM_UPDATE_ELEMENT_TEXT,
                TDE_FOOTER as _,
                footer_wchar.as_ptr() as _,
            );
        }
    }

    /** Set the expanded information text */
    pub fn set_expanded_information(&mut self, expanded_information: &str) {
        if self.dialog_hwnd == null_mut() {
            return;
        }
        let expanded_information_wchar = U16CString::from_str(expanded_information).unwrap();
        unsafe {
            SendMessageA(
                self.dialog_hwnd,
                TDM_UPDATE_ELEMENT_TEXT,
                TDE_EXPANDED_INFORMATION as _,
                expanded_information_wchar.as_ptr() as _,
            );
        }
    }
}

#[cfg(not(windows))]
impl TaskDialogConfig {
    pub fn enable_process_bar(&mut self, _marquee: bool) {}
    pub fn set_process_bar_marquee(&mut self, _enable: bool, _time: isize) {}
    pub fn set_process_bar(&mut self, _percentage: usize) {}
    pub fn set_content(&mut self, content: &str) {}
    pub fn set_main_instruction(&mut self, main_instruction: &str) {}
    pub fn set_footer(&mut self, footer: &str) {}
    pub fn set_expanded_information(&mut self, expanded_information: &str) {}
}

pub struct TaskDialogButton {
    pub id: i32,
    pub text: String,
}

pub struct TaskDialogResult {
    pub button_id: i32,
    pub radio_button_id: i32,
    pub checked: bool,
}

impl Default for TaskDialogResult {
    fn default() -> Self {
        TaskDialogResult {
            button_id: 0,
            radio_button_id: 0,
            checked: false,
        }
    }
}

/** Show task dialog */
#[cfg(windows)]
pub fn show_task_dialog(conf: &mut TaskDialogConfig) -> Result<TaskDialogResult, Error> {
    let mut result = TaskDialogResult::default();
    let conf_ptr: *mut TaskDialogConfig = conf;
    let conf_long_ptr = conf_ptr as isize;

    use std::ffi::OsStr;
    use std::iter::once;
    use std::mem;
    use std::os::windows::ffi::OsStrExt;
    /** Convert string to wide string */
    fn to_os_string(text: &String) -> Vec<u16> {
        OsStr::new(text).encode_wide().chain(once(0)).collect()
    }

    fn from_wide_ptr(ptr: *const u16) -> String {
        use std::ffi::OsString;
        use std::os::windows::ffi::OsStringExt;
        unsafe {
            let len = (0..std::isize::MAX)
                .position(|i| *ptr.offset(i) == 0)
                .unwrap();
            let slice = std::slice::from_raw_parts(ptr, len);
            OsString::from_wide(slice).to_string_lossy().into_owned()
        }
    }

    let ret = unsafe {
        // Call GetModuleHandleA on conf.instance is null
        let instance = if conf.instance == null_mut() {
            GetModuleHandleA(std::ptr::null())
        } else {
            conf.instance
        };

        // Some text
        let window_title: Vec<u16> = to_os_string(&conf.window_title);
        let main_instruction: Vec<u16> = to_os_string(&conf.main_instruction);
        let content: Vec<u16> = to_os_string(&conf.content);
        let verification_text: Vec<u16> = to_os_string(&conf.verification_text);
        let expanded_information: Vec<u16> = to_os_string(&conf.expanded_information);
        let expanded_control_text: Vec<u16> = to_os_string(&conf.expanded_control_text);
        let collapsed_control_text: Vec<u16> = to_os_string(&conf.collapsed_control_text);
        let footer: Vec<u16> = to_os_string(&conf.footer);

        // Buttons
        let mut buttons: Vec<TASKDIALOG_BUTTON> = Vec::new();
        let mut btn_text: Vec<Vec<u16>> = Vec::new();
        for v in conf.buttons.iter() {
            btn_text.push(to_os_string(&v.text));
        }
        for i in 0..conf.buttons.len() {
            buttons.push(TASKDIALOG_BUTTON {
                nButtonID: conf.buttons[i].id,
                pszButtonText: btn_text[i].as_ptr(),
            });
        }

        // Radio Buttons
        let mut radio_buttons: Vec<TASKDIALOG_BUTTON> = Vec::new();
        let mut radio_btn_text: Vec<Vec<u16>> = Vec::new();
        for v in conf.radio_buttons.iter() {
            radio_btn_text.push(to_os_string(&v.text));
        }
        for i in 0..conf.radio_buttons.len() {
            radio_buttons.push(TASKDIALOG_BUTTON {
                nButtonID: conf.radio_buttons[i].id,
                pszButtonText: radio_btn_text[i].as_ptr(),
            });
        }

        // ICON
        let mut u1: TASKDIALOGCONFIG_u1 = Default::default();
        let mut u2: TASKDIALOGCONFIG_u2 = Default::default();
        if conf.main_icon != null_mut() {
            core::ptr::write(u1.pszMainIcon_mut(), conf.main_icon as *const u16);
        }
        if conf.footer_icon != null_mut() {
            core::ptr::write(u2.pszFooterIcon_mut(), conf.footer_icon as *const u16);
        }

        extern "system" fn callback(
            hwnd: HWND,
            msg: UINT,
            _w_param: WPARAM,
            _l_param: LPARAM,
            lp_ref_data: LONG_PTR,
        ) -> HRESULT {
            if msg == TDN_CREATED {
                unsafe {
                    let conf = std::mem::transmute::<isize, *mut TaskDialogConfig>(lp_ref_data);
                    (*conf).dialog_hwnd = hwnd;
                }
            } else if msg == TDN_DESTROYED {
                unsafe {
                    let conf = std::mem::transmute::<isize, *mut TaskDialogConfig>(lp_ref_data);
                    (*conf).is_destroyed = true;
                }
            } else if msg == TDN_HYPERLINK_CLICKED {
                unsafe {
                    let conf = std::mem::transmute::<isize, *mut TaskDialogConfig>(lp_ref_data);
                    let link = from_wide_ptr(_l_param as *const u16);
                    let callback_func = (*conf).hyperlinkclicked_callback;
                    callback_func(link);
                }
            }
            0
        }

        let config = TASKDIALOGCONFIG {
            cbSize: mem::size_of::<TASKDIALOGCONFIG>() as UINT,
            hwndParent: conf.parent,
            hInstance: instance,
            dwFlags: conf.flags,
            dwCommonButtons: conf.common_buttons,
            pszWindowTitle: window_title.as_ptr(),
            pszMainInstruction: main_instruction.as_ptr(),
            pszContent: content.as_ptr(),
            pszVerificationText: verification_text.as_ptr(),
            pszExpandedInformation: expanded_information.as_ptr(),
            pszExpandedControlText: expanded_control_text.as_ptr(),
            pszCollapsedControlText: collapsed_control_text.as_ptr(),
            pszFooter: footer.as_ptr(),
            cButtons: buttons.len() as UINT,
            pButtons: buttons.as_slice().as_ptr(),
            nDefaultButton: conf.default_button,
            cRadioButtons: radio_buttons.len() as UINT,
            pRadioButtons: radio_buttons.as_slice().as_ptr(),
            nDefaultRadioButton: conf.default_radio_buttons,
            u1,
            u2,
            pfCallback: Some(callback),
            lpCallbackData: conf_long_ptr,
            cxWidth: 0,
        };

        // Result
        let mut verify: BOOL = FALSE;
        let dialog_result = TaskDialogIndirect(
            &config,
            &mut result.button_id,
            &mut result.radio_button_id,
            &mut verify,
        );
        result.checked = verify != 0;
        dialog_result
    };
    if ret != 0 {
        Err(Error::last_os_error())
    } else {
        Ok(result)
    }
}

/** Show message dialog, the dialog have only the OK button */
#[cfg(windows)]
pub fn show_msg_dialog(
    title: &str,
    main_instruction: &str,
    content: &str,
    icon: LPWSTR,
) -> Option<Error> {
    let mut conf = TaskDialogConfig {
        common_buttons: TDCBF_OK_BUTTON,
        window_title: title.to_string(),
        main_instruction: main_instruction.to_string(),
        content: content.to_string(),
        main_icon: icon,
        ..Default::default()
    };
    show_task_dialog(&mut conf).err()
}

#[cfg(not(windows))]
pub fn show_task_dialog(_conf: &TaskDialogConfig) -> Result<TaskDialogResult, Error> {
    Ok(TaskDialogResult::default())
}

#[cfg(not(windows))]
pub fn show_msg_dialog(
    _title: &str,
    _main_instruction: &str,
    _content: &str,
    _icon: *mut u16,
) -> Option<Error> {
    None
}
