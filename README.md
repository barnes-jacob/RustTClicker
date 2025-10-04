# Rust-T AutoClicker

I made this for my own personal use and to experiment with Rust/Tauri.  
I based the design on: [https://github.com/oriash93/AutoClicker](https://github.com/oriash93/AutoClicker)

I may soon add several additional features, e.g. system tray icon, customizable hotkey, etc.

![Rust-T AutoClicker Screenshot](https://github.com/barnes-jacob/RustTClicker/blob/main/rustTss.png)

---

## Build & Test

### Prerequisites
- [Rust](https://rustup.rs/) toolchain installed  
- Tauri CLI  
  ```bash
  cargo install tauri-cli
  # or
  npm i -g @tauri-apps/cli
  ```
- On Linux: install `libgtk-3-dev`, `libwebkit2gtk` and related deps

---

### Clone & Run

```bash
git clone https://github.com/barnes-jacob/RustTClicker.git
cd RustTClicker
cargo tauri dev
```

This launches the app in dev mode with hot reload.

---

### Build Release

```bash
cargo tauri build
```

Binaries and installers are generated in:
```
src-tauri/target/release/bundle/
```

---

### Usage

- **F6** toggles clicking on and off  
- Configure interval, random jitter, and mouse button through the UI  
- Total clicks are tracked per session

---

### First-time Run Notes

**Windows:** If SmartScreen warns about an unknown publisher, click “More info” → “Run anyway”.  
**macOS:** Right-click the app and select “Open” the first time to bypass unidentified developer warnings.  
**Linux:** Make the AppImage executable with `chmod +x` if needed.
