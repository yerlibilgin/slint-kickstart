#![windows_subsystem = "windows"]
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    // You can still react to the state change in Rust if needed
    let ui_handle = ui.as_weak();
    ui.window().on_close_requested(move || {
        println!("App closing...");
        slint::CloseRequestResponse::HideWindow
    });

    ui.run()
}
