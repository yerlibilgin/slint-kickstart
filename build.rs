use std::ffi::OsString;

fn main() {
  slint_build::compile("ui/app-window.slint").expect("Slint compilation failed");

  // Embed Windows Resource (Icon)
  if std::env::var("CARGO_CFG_WINDOWS").is_ok() {
    let app_name = "SlintTest";

    let macros = vec![
      format!("VERSION_MAJOR={}", 2),
      format!("VERSION_MINOR={}", 5),
      // This is the critical part: it must result in "APP_NAME=\"SlintTest\""
      format!("APP_NAME=\"{}\"", app_name),
    ];

    // If using embed_resource::compile:
    embed_resource::compile("assets/resources.rc", macros);
  }
}