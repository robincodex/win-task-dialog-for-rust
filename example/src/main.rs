extern crate win_task_dialog_for_rust;

use win_task_dialog_for_rust::*;

fn main() {
    let conf = TaskDialogConfig {
        parent: std::ptr::null_mut(),
        instance: std::ptr::null_mut(),
        flags: TDF_USE_COMMAND_LINKS,
        common_buttons: TDCBF_OK_BUTTON | TDCBF_CANCEL_BUTTON,
        window_title: "WindowTitle标题".to_string(),
        main_instruction: "MainInstruction主要".to_string(),
        content: "内容".to_string(),
        verification_text: "VerificationText".to_string(),
        expanded_information: "".to_string(),
        expanded_control_text: "".to_string(),
        collapsed_control_text: "".to_string(),
        footer: "".to_string(),
        buttons: vec![
            TaskDialogButton {
                id: 10,
                text: "One".to_string(),
            },
            TaskDialogButton {
                id: 11,
                text: "Two".to_string(),
            },
            TaskDialogButton {
                id: 13,
                text: "可以".to_string(),
            },
        ],
        default_button: 11,
        radio_buttons: vec![],
        default_radio_buttons: 0,
    };
    let result = show_task_dialog(&conf).unwrap();
    println!(
        "{} {} {}",
        result.button_id, result.radio_button_id, result.checked,
    )
}
