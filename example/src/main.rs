extern crate win_task_dialog;

use win_task_dialog::*;

fn main() {
    let conf = TaskDialogConfig {
        flags: TDF_USE_COMMAND_LINKS,
        common_buttons: TDCBF_OK_BUTTON | TDCBF_CANCEL_BUTTON,
        window_title: "Title 标题".to_string(),
        main_instruction: "Привет".to_string(),
        content: "こんにちは".to_string(),
        verification_text: "VerificationText".to_string(),
        footer: "footer".to_string(),
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
        ..Default::default()
    };
    let result = show_task_dialog(&conf).unwrap();
    println!(
        "{} {} {}",
        result.button_id, result.radio_button_id, result.checked,
    );

    show_msg_dialog("Title", "Hi", "Info", TD_INFORMATION_ICON);
    show_msg_dialog("Title", "!!!", "Error", TD_ERROR_ICON);
}
