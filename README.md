# Rule34Video Tauri App

**🚀 Current release:** v1.0.3 • **Updated:** 2026-05-17

A privacy-forward native wrapper for **rule34video.com** built with **Tauri 2** and **Rust**.

This repository packages the Rule34Video website into a lightweight native shell that adds privacy and platform-native behavior without changing the familiar web experience.

The app includes:

- Ad and tracker blocking
- Deep linking support
- Download handling
- Child window management and popup handling
- System tray integration on desktop platforms
- Cross-platform builds for Windows, macOS, Linux, Android, and iOS

---


## Overview

Rule34Video Tauri App is a native application shell for the web platform at `rule34video.com`.
The app uses Tauri to wrap the website and add native capabilities that are not available in a browser alone.

This repository includes both the Tauri source and native platform build helpers.

---

## Supported Platforms

- **Windows** (`x86_64-pc-windows-gnu`)
- **macOS** (`x86_64-apple-darwin` via osxcross)
- **Linux** (`x86_64-unknown-linux-gnu`)
- **Android** (`aarch64-linux-android`, `armv7-linux-androideabi`, `i686-linux-android`, `x86_64-linux-android`)
- **iOS** (manual Xcode-based build via documentation)

---

## Quick Start

### Local development

```bash
git clone https://github.com/PhantomNimbi/Rule34Video-Tauri-App.git
cd Rule34Video-Tauri-App
cargo install tauri-cli --version "^2" --locked
cd src-tauri
cargo tauri dev
```

### Build the app locally

From the repository root:

```bash
cd src-tauri
cargo tauri build
```

---

## Build locally

From the repository root, use the official Tauri commands in `src-tauri`.

```bash
cd src-tauri
cargo tauri build
```

For platform-specific builds, use the official Tauri targets:

- Windows: `cargo tauri build`
- macOS: `cargo tauri build`
- Linux: `cargo tauri build`
- Android: `cargo tauri android build --apk -t aarch64 armv7 i686 x86_64`
- iOS: `cargo tauri ios build`

Use `--release` or other `cargo tauri` args as needed.

---

## Project Structure

- `src-tauri/` — Tauri application source code and Rust build configuration
- `build.sh` — unified build helper for Linux, macOS, Android, iOS, and Windows
- `build.bat` — Windows native build fallback

---

## License

This repository includes a `LICENSE` file. Review it for details on usage, distribution, and contribution terms.
