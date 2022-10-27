extern crate win_task_dialog;

use std::thread;
use std::time::Duration;
use win_task_dialog::*;

fn callback(link: String) {
    println!("{}", link);
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
        hyperlinkclicked_callback: callback,
        ..TaskDialogConfig::default()
    };

    let result = show_task_dialog(&mut conf).unwrap();
    println!(
        "{} {} {}",
        result.button_id, result.radio_button_id, result.checked,
    );

    show_process_bar_marquee();
    show_process_bar();

    show_msg_dialog("Title", "Hi", "Info", TD_INFORMATION_ICON);
    show_msg_dialog("Title", "!!!", "Error", TD_ERROR_ICON);
}

// Process Bar Marquee
fn show_process_bar_marquee() {
    let mut conf = TaskDialogConfig {
        window_title: "Process Bar".to_string(),
        main_instruction: "Process Bar Marquee".to_string(),
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
        (*conf).set_process_bar_marquee(true, 1);
        thread::sleep(Duration::from_secs(3));
        (*conf).set_process_bar_marquee(false, 1);
    });

    show_task_dialog(&mut conf).unwrap();
}

// Process Bar (Not Marquee)
fn show_process_bar() {
    let mut conf = TaskDialogConfig {
        window_title: "Process Bar".to_string(),
        main_instruction: "Process Bar (Not Marquee)".to_string(),
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
