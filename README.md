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

---

### Clone & Run

```bash
git clone https://github.com/barnes-jacob/RustTClicker.git
cd RustTClicker
cargo tauri dev
```

This launches the app in dev mode.

---

### Build Release

```bash
cargo tauri build
```

Binaries and installers are generated in:
```
src-tauri/target/release/bundle/
```

