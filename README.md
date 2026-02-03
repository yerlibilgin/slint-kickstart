# SlintTest - Cross-Platform Rust GUI Boilerplate

This is a minimal, boilerplate-free starter template for building cross-platform desktop applications (Windows & macOS) using [Rust](https://www.rust-lang.org/) and [Slint](https://slint.dev/).

It handles the common headaches of setting up application icons, Windows resources (`.rc`), and build configurations so you can focus on your UI and logic.

## Features

*   **Cross-Platform:** Configured for both Windows and macOS.
*   **Slint UI:** Uses Slint for a modern, lightweight, and native-looking UI.
*   **Windows Resources:** Automatically embeds application icons and metadata (version, app name) into the Windows executable using `embed-resource`.
*   **MacOS Bundle:** Ready for `cargo-bundle` to create `.app` bundles with icons.
*   **Optimized Release Profile:** `Cargo.toml` is pre-configured for small binary sizes (`opt-level = 'z'`, `strip = true`, `lto = true`).
*   **No Console Window:** Configured to run as a GUI app (no popping command prompt on Windows).

## Prerequisites

*   Rust (latest stable)
*   [ImageMagick](https://imagemagick.org/) (for generating icons)

## Project Structure

```
slinttest/
├── assets/             # Icons and resource files
│   ├── icon.png        # Source icon (start with this)
│   ├── resources.rc    # Windows resource script
│   └── ...             # Generated icons (icon.ico, 32.png, etc.)
├── src/
│   └── main.rs         # Entry point
├── ui/
│   └── app-window.slint # Slint UI definition
├── build.rs            # Build script for compiling Slint and embedding resources
└── Cargo.toml          # Dependencies and build configuration
```

## Generating Icons

To generate the necessary icon formats (`.ico` for Windows, `.png` sizes for macOS) from a single source image, use ImageMagick.

1.  Place your high-resolution source icon (e.g., 1024x1024) at `assets/icon.png`.
2.  Run the following commands in your terminal:

```bash
# Generate Windows .ico file (contains multiple sizes)
magick convert assets/icon.png -define icon:auto-resize=256,128,64,48,32,16 assets/icon.ico
```

## Building and Running

### Development
```bash
cargo run
```

### Release (Windows)
Produces a standalone `.exe` with the icon embedded.
```bash
cargo build --release
```
The output will be in `target/release/slinttest.exe`.

### Release (macOS)
To create a proper `.app` bundle:

1.  Install `cargo-bundle`:
    ```bash
    cargo install cargo-bundle
    ```
2.  Build the bundle:
    ```bash
    cargo bundle --release
    ```
The output will be in `target/release/bundle/osx/SlintTest.app`.

## Customization

*   **UI:** Edit `ui/app-window.slint` to design your interface.
*   **App Name & Version:**
    *   Update `Cargo.toml` (`name`, `version`, `package.metadata.bundle`).
    *   Update `build.rs` (`app_name`, `VERSION_MAJOR`, `VERSION_MINOR`) to match.

## License

MIT
