fn main() {
  slint_build::compile("ui/app-window.slint").expect("Slint compilation failed");

  // Embed Windows Resource (Icon & Version Info)
  if std::env::var("CARGO_CFG_WINDOWS").is_ok() {
    // Get package name and version from Cargo
    let pkg_name = std::env::var("CARGO_PKG_NAME").unwrap_or("SlintTest".to_string());
    let major = std::env::var("CARGO_PKG_VERSION_MAJOR").unwrap_or("0".to_string());
    let minor = std::env::var("CARGO_PKG_VERSION_MINOR").unwrap_or("1".to_string());
    let patch = std::env::var("CARGO_PKG_VERSION_PATCH").unwrap_or("0".to_string());
    let version_string = format!("{}.{}.{}", major, minor, patch);
    let file_name = format!("{}.exe", pkg_name);

    // Define the macros for the .rc file
    // The `embed-resource` crate handles quoting strings, so we don't add them manually.
    let macros = vec![
      // Version numbers (passed as is)
      format!("RC_VERSION_MAJOR={}", major),
      format!("RC_VERSION_MINOR={}", minor),
      format!("RC_VERSION_PATCH={}", patch),
      format!("RC_VERSION_STRING={}", version_string),
      format!("RC_PROJECT_NAME={}", pkg_name),
      format!("RC_FILE_NAME={}", file_name),
    ];

    // Compile the resource file with the defined macros
    embed_resource::compile("assets/resources.rc", macros);
  }
}
