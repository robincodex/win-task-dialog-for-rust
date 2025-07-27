extern crate win_task_dialog;

use std::thread;
use std::time::Duration;
use win_task_dialog::*;

fn hyperlink_callback(context: &str) {
    println!("hyperlink_callback: {}", context);
}

unsafe extern "system" fn callback(
    hwnd: HWND,
    msg: TASKDIALOG_NOTIFICATIONS,
    w_param: WPARAM,
    l_param: LPARAM,
    ref_data: *mut TaskDialogConfig,
) -> HRESULT {
    println!(
        "callback: hwnd={:?} msg={} wparam={:#X} lparam={:#X} ref_data={:?}",
        hwnd, msg.0, w_param.0, l_param.0, ref_data
    );

    S_OK
}

fn main() {
    let mut conf = TaskDialogConfig {
        flags: TDF_USE_COMMAND_LINKS | TDF_ENABLE_HYPERLINKS,
        common_buttons: TDCBF_OK_BUTTON | TDCBF_CANCEL_BUTTON,
        window_title: "Title 标题".to_string(),
        main_instruction: "Привет".to_string(),
        content: "こんにちは".to_string(),
        verification_text: "VerificationText".to_string(),
        footer: "footer\n<a href=\"http://example.com\">example.com</a>".to_string(),
        buttons: vec![
            TaskDialogButton {
                id: 10,
                text: "One".to_string(),
            },
            TaskDialogButton {
                id: 11,
                text: "贰".to_string(),
            },
        ],
        default_button: 11,
        radio_buttons: vec![
            TaskDialogButton {
                id: 10,
                text: "Option 1".to_string(),
            },
            TaskDialogButton {
                id: 11,
                text: "Option 2".to_string(),
            },
        ],
        main_icon: TD_SHIELD_ICON,
        footer_icon: TD_INFORMATION_ICON,
        hyperlink_callback: Some(hyperlink_callback),
        callback: Some(callback),
        ..TaskDialogConfig::default()
    };

    let result = show_task_dialog(&mut conf).unwrap();
    println!(
        "{} {} {}",
        result.button_id, result.radio_button_id, result.checked,
    );

    show_dynamic_text_dialog();
    show_process_bar_marquee();
    show_process_bar();

    show_msg_dialog("Title", "Hi", "Info", TD_INFORMATION_ICON);
    show_msg_dialog("Title", "!!!", "Error", TD_ERROR_ICON);

    page_navigation();
}

// Show dynamic text dialog
fn show_dynamic_text_dialog() {
    let mut conf = TaskDialogConfig {
        window_title: "Dynamic Text".to_string(),
        main_instruction: "Main".to_string(),
        content: "Content".to_string(),
        footer: "Footer".to_string(),
        expanded_information: "Info".to_string(),
        ..Default::default()
    };

    let conf_ptr: *mut TaskDialogConfig = &mut conf;
    let conf_long_ptr = conf_ptr as isize;
    thread::spawn(move || unsafe {
        let conf = std::mem::transmute::<isize, *mut TaskDialogConfig>(conf_long_ptr);
        for i in 1..4 {
            thread::sleep(Duration::from_secs(1));
            (*conf).set_main_instruction(format!("Main {}", i).as_str());
            (*conf).set_content(format!("Content {}", i).as_str());
            (*conf).set_footer(format!("Footer {}", i).as_str());
            (*conf).set_expanded_information(format!("Info {}", i).as_str());
        }
        (*conf).set_main_instruction("Main !!!");
        (*conf).set_content("Content !!!");
        (*conf).set_footer("Footer !!!");
        (*conf).set_expanded_information("Info !!!");
    });

    show_task_dialog(&mut conf).unwrap();
}

// Process Bar Marquee
fn show_process_bar_marquee() {
    let mut conf = TaskDialogConfig {
        window_title: "Process Bar".to_string(),
        main_instruction: "Process Bar Marquee 1".to_string(),
        ..Default::default()
    };
    conf.enable_process_bar(true);

    let conf_ptr: *mut TaskDialogConfig = &mut conf;
    let conf_long_ptr = conf_ptr as isize;
    thread::spawn(move || unsafe {
        thread::sleep(Duration::from_secs(1));
        let conf = std::mem::transmute::<isize, *mut TaskDialogConfig>(conf_long_ptr);
        (*conf).set_process_bar_marquee(true, 100);
        thread::sleep(Duration::from_secs(5));
        (*conf).set_main_instruction("Process Bar Marquee 2");
        (*conf).set_process_bar_marquee(true, 1);
        thread::sleep(Duration::from_secs(3));
        (*conf).set_main_instruction("Process Bar Marquee Stop");
        (*conf).set_process_bar_marquee(false, 1);
    });

    show_task_dialog(&mut conf).unwrap();
}

// Process Bar (Not Marquee)
fn show_process_bar() {
    let mut conf = TaskDialogConfig {
        window_title: "Process Bar".to_string(),
        main_instruction: "Process Bar (Not Marquee)".to_string(),
        content: "".to_string(),
        ..Default::default()
    };
    conf.enable_process_bar(false);

    let conf_ptr: *mut TaskDialogConfig = &mut conf;
    let conf_long_ptr = conf_ptr as isize;
    thread::spawn(move || unsafe {
        let conf = std::mem::transmute::<isize, *mut TaskDialogConfig>(conf_long_ptr);
        for i in 0..101 {
            if (*conf).is_destroyed {
                return;
            }
            thread::sleep(Duration::from_millis(20));
            (*conf).set_process_bar(i);
        }
    });

    show_task_dialog(&mut conf).unwrap();
}

fn page_navigation() {
    unsafe extern "system" fn page1_callback(
        _: HWND,
        msg: TASKDIALOG_NOTIFICATIONS,
        w_param: WPARAM,
        _: LPARAM,
        ref_data: *mut TaskDialogConfig,
    ) -> HRESULT {
        if msg == TDN_NAVIGATED {
        } else if msg == TDN_BUTTON_CLICKED {
            // TDN_BUTTON_CLICKED

            // Note that lifetime is limited in Rust objects
            // and we cannot make new struct in stack here.
            // Instead we should modify `ref_data`.
            if w_param.0 == 1776 {
                (*ref_data).window_title = "Page #1".to_owned();
                (*ref_data).main_instruction = "Page #1".to_owned();
                (*ref_data).buttons = vec![TaskDialogButton {
                    id: 1777,
                    text: "Continue".to_owned(),
                }];
                (*ref_data).navigate_page(&mut *ref_data);
                return S_FALSE;
            } else if w_param.0 == 1777 {
                (*ref_data).window_title = "Page #2".to_owned();
                (*ref_data).main_instruction = "Page #2".to_owned();
                (*ref_data).buttons = vec![TaskDialogButton {
                    id: 1776,
                    text: "Back to page #1".to_owned(),
                }];
                (*ref_data).navigate_page(&mut *ref_data);
                return S_FALSE;
            }
        }

        S_OK
    }

    let mut conf = TaskDialogConfig {
        window_title: "Page Navigation".to_owned(),
        main_instruction: "Page #1".to_owned(),
        callback: Some(page1_callback),
        common_buttons: TDCBF_CLOSE_BUTTON,
        buttons: vec![TaskDialogButton {
            id: 1777,
            text: "Continue".to_owned(),
        }],
        ..Default::default()
    };

    show_task_dialog(&mut conf).unwrap();
}
