# Windows Task Dialog For Rust

Simply call the Win32 API `TaskDialogIndirect`.

# Dependencies

```toml
[dependencies]
win-task-dialog-for-rust = "0.1.0"

[build-dependencies]
embed-resource = "1.6"
```

> NOTE: You must be [Enabling Visual Styles](https://docs.microsoft.com/en-us/windows/win32/controls/cookbook-overview#using-comctl32dll-version-6-in-an-application-that-uses-only-standard-extensions) and using [embed-resource](https://github.com/nabijaczleweli/rust-embed-resource) to change manifest.

# Example

```rust
extern crate win_task_dialog_for_rust;

use win_task_dialog_for_rust::*;

fn main() {
    let conf = TaskDialogConfig {
        parent: std::ptr::null_mut(),
        instance: std::ptr::null_mut(),
        flags: TDF_USE_COMMAND_LINKS,
        common_buttons: TDCBF_OK_BUTTON | TDCBF_CANCEL_BUTTON,
        window_title: "Title 标题".to_string(),
        main_instruction: "Привет".to_string(),
        content: "こんにちは".to_string(),
        verification_text: "VerificationText".to_string(),
        expanded_information: "".to_string(),
        expanded_control_text: "".to_string(),
        collapsed_control_text: "".to_string(),
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
        default_radio_buttons: 0,
        main_icon: TD_SHIELD_ICON,
        footer_icon: TD_INFORMATION_ICON,
    };
    let result = show_task_dialog(&conf).unwrap();
    println!(
        "{} {} {}",
        result.button_id, result.radio_button_id, result.checked,
    )
}

```

# Example Screenshot

![Screenshot](https://user-images.githubusercontent.com/8408783/108849894-a50aa700-761d-11eb-8e19-ccd7aea12ba6.png)