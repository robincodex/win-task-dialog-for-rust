# Windows Task Dialog For Rust

[![Crates.io](https://img.shields.io/crates/v/win-task-dialog)](https://crates.io/crates/win-task-dialog)
[![docs.rs](https://docs.rs/win-task-dialog/badge.svg)](https://docs.rs/win-task-dialog)

Simply call the Win32 API `TaskDialogIndirect` in Rust.

> ~~Can be safely call on not windows platforms, but it has no effect.~~
> 
> After version 1.0.0, it can no longer run properly on Linux. Please use `cfg(windows)` to mark the usage.

## Dependencies

```toml
[target.'cfg(windows)'.dependencies]
win-task-dialog = "1.0.0"

[build-dependencies]
embed-resource = "2.1"
```

> NOTE: You must be [Enabling Visual Styles](https://docs.microsoft.com/en-us/windows/win32/controls/cookbook-overview#using-comctl32dll-version-6-in-an-application-that-uses-only-standard-extensions) and using [embed-resource](https://github.com/nabijaczleweli/rust-embed-resource) to change manifest.

## Example

See [example/src/main.rs](https://github.com/RobinCodeX/win-task-dialog-for-rust/blob/main/example/src/main.rs)

## About Process Bar

I used a very unsafe way to implement these functions and I didn't have a good way to solve this problem.

## Screenshot

![Screenshot](https://user-images.githubusercontent.com/8408783/108849894-a50aa700-761d-11eb-8e19-ccd7aea12ba6.png)
