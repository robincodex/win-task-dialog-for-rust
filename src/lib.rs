#[cfg(windows)]
extern crate winapi;

use std::io::Error;
use std::iter::once;
use std::mem;
use std::ptr::null_mut;
use winapi::ctypes::*;
use winapi::shared::minwindef::*;
use winapi::shared::windef::HWND;
use winapi::um::commctrl::{
    TASKDIALOGCONFIG_u1, TASKDIALOGCONFIG_u2, TaskDialogIndirect, TASKDIALOGCONFIG,
    TASKDIALOG_BUTTON, TASKDIALOG_COMMON_BUTTON_FLAGS, TASKDIALOG_FLAGS,
};
use winapi::um::libloaderapi::GetModuleHandleA;
use winapi::um::winnt::LPWSTR;

pub use winapi::um::commctrl::{
    TDCBF_CANCEL_BUTTON, TDCBF_CLOSE_BUTTON, TDCBF_NO_BUTTON, TDCBF_OK_BUTTON, TDCBF_RETRY_BUTTON,
    TDCBF_YES_BUTTON, TDF_ALLOW_DIALOG_CANCELLATION, TDF_CALLBACK_TIMER, TDF_CAN_BE_MINIMIZED,
    TDF_ENABLE_HYPERLINKS, TDF_EXPANDED_BY_DEFAULT, TDF_EXPAND_FOOTER_AREA,
    TDF_NO_DEFAULT_RADIO_BUTTON, TDF_NO_SET_FOREGROUND, TDF_POSITION_RELATIVE_TO_WINDOW,
    TDF_RTL_LAYOUT, TDF_SHOW_MARQUEE_PROGRESS_BAR, TDF_SHOW_PROGRESS_BAR, TDF_SIZE_TO_CONTENT,
    TDF_USE_COMMAND_LINKS, TDF_USE_COMMAND_LINKS_NO_ICON, TDF_USE_HICON_FOOTER, TDF_USE_HICON_MAIN,
    TDF_VERIFICATION_FLAG_CHECKED, TD_ERROR_ICON, TD_INFORMATION_ICON, TD_SHIELD_ICON,
    TD_WARNING_ICON,
};

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
    pub default_button: c_int,
    pub radio_buttons: Vec<TaskDialogButton>,
    pub default_radio_buttons: c_int,
    pub main_icon: LPWSTR,
    pub footer_icon: LPWSTR,
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
        }
    }
}

pub struct TaskDialogButton {
    pub id: c_int,
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
pub fn show_task_dialog(conf: &TaskDialogConfig) -> Result<TaskDialogResult, Error> {
    let mut result = TaskDialogResult::default();
    let ret = unsafe {
        use std::ffi::OsStr;
        use std::os::windows::ffi::OsStrExt;
        /** Convert string to wide string */
        fn to_os_string(text: &String) -> Vec<u16> {
            OsStr::new(text).encode_wide().chain(once(0)).collect()
        }

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
            pfCallback: None,
            lpCallbackData: 0,
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
    let conf = TaskDialogConfig {
        common_buttons: TDCBF_OK_BUTTON,
        window_title: title.to_string(),
        main_instruction: main_instruction.to_string(),
        content: content.to_string(),
        main_icon: icon,
        ..Default::default()
    };
    show_task_dialog(&conf).err()
}

#[cfg(not(windows))]
pub fn show_task_dialog(conf: &DialogConfig) -> Result<TaskDialogResult, Error> {
    TaskDialogResult::default()
}
