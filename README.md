# Rule34Video Tauri App

**🚀 Current release:** v1.0.2 • **Updated:** 2026-05-09

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

## Contents

- [Rule34Video Tauri App](#virtual-customs-tauri-app)
  - [Contents](#contents)
  - [Overview](#overview)
  - [Supported Platforms](#supported-platforms)
  - [Quick Start](#quick-start)
    - [Local development](#local-development)
    - [Build the app locally](#build-the-app-locally)
  - [Versioning helper](#versioning-helper)
  - [Project Structure](#project-structure)
  - [Documentation](#documentation)
  - [Contributing](#contributing)
  - [License](#license)

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
git clone https://github.com/PhantomNimbi/virtual-customs-tauri-app.git
cd virtual-customs-tauri-app
cargo install tauri-cli --version "^2" --locked
cd src-tauri
cargo tauri dev
```

> If you are developing on mobile targets, follow the platform-specific setup guides in `docs/platforms/`.

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

## Versioning helper

This repository includes a safe semantic version helper for release metadata and app manifests:

- `.github/scripts/auto-version.sh` — calculates the next semantic version, updates `src-tauri/Cargo.toml` and `src-tauri/tauri.conf.json`, and can optionally create a git commit and tag.
- Conventional Commits determine bump type: `feat` → minor, `fix` → patch, `BREAKING CHANGE` → major.
- Documentation-only or tooling-only changes are handled as prerelease updates, so routine docs work does not force a public release.

Example usage:

```bash
./.github/scripts/auto-version.sh --dry-run
./.github/scripts/auto-version.sh
./.github/scripts/auto-version.sh --commit
```

---

## Project Structure

- `src-tauri/` — Tauri application source code and Rust build configuration
- `build.sh` — unified build helper for Linux, macOS, Android, iOS, and Windows
- `build.bat` — Windows native build fallback
- `docs/` — project documentation and platform guides

---

## Documentation

Detailed documentation is available in the `docs/` folder:

- `docs/Getting-Started.md`
- `docs/Installation.md`
- `docs/Features.md`
- `docs/platforms/Windows.md`
- `docs/platforms/macOS.md`
- `docs/platforms/Linux.md`
- `docs/platforms/Android.md`
- `docs/platforms/iOS.md`

---

## Contributing

See `docs/Contributing.md` for contribution guidelines, code style, and project workflow.

---

## License

This repository includes a `LICENSE` file. Review it for details on usage, distribution, and contribution terms.
