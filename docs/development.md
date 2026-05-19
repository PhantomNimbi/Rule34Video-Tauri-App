# 🛠️ Development

> How to build, test, and contribute to the Rule34Video Tauri App.

---

## 📋 Prerequisites

| Tool | Version | Purpose |
|---|---|---|
| **Rust** | 1.85+ (edition 2021) | Backend compilation |
| **Tauri CLI** | ^2 | Build, dev, and package commands |
| **Node.js** | (not required) | No frontend — this app wraps a remote site directly |

### Platform-specific requirements

- **Windows**: WebView2 Runtime (included in Windows 11, available via update on Windows 10)
- **macOS**: Xcode Command Line Tools
- **Linux**: WebKitGTK, libsoup, and other Tauri v2 Linux dependencies
- **Android**: Android SDK + NDK, Java 17+, Gradle
- **iOS**: Xcode 15+, CocoaPods

See the [official Tauri v2 prerequisites guide](https://v2.tauri.app/start/prerequisites/) for full details.

---

## 🚀 Quick start

```sh
# Clone the repository
git clone https://github.com/your-username/rule34video-tauri-app.git
cd rule34video-tauri-app

# Desktop development
cargo tauri dev

# Android development
cargo tauri dev --target aarch64-linux-android

# iOS development
cargo tauri dev --target aarch64-apple-ios

# Production build (desktop)
cargo tauri build

# Production build (mobile)
cargo tauri build --target aarch64-linux-android
```

---

## 📁 Project structure

```sql
rule34video-tauri-app/
├── src-tauri/
│   ├── src/
│   │   ├── lib.rs                         # App setup, command registration
│   │   ├── main.rs                        # Binary entrypoint
│   │   └── ext/
│   │       ├── adblock.rs                 # Adblock engine + JS injection
│   │       ├── adblock_bundled.txt        # ~900 bundled filter rules
│   │       ├── webview_intercept.rs       # WebView2 native interception (Win)
│   │       ├── navigation.rs              # Link handling + init script builder
│   │       ├── child_windows.rs           # Child webview windows (desktop)
│   │       ├── context_menu.rs            # Native right-click menu (desktop)
│   │       ├── downloads.rs               # Download interception
│   │       ├── tray.rs                    # System tray (desktop)
│   │       ├── global_shortcuts.rs        # Global shortcuts (desktop)
│   │       ├── webnotifications.rs        # Notification permissions
│   │       ├── cloudfare.rs               # Anti-bot User-Agent
│   │       └── universal_deep_link.rs     # Deep link handler
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── tauri.windows.conf.json
│   ├── tauri.macos.conf.json
│   ├── tauri.linux.conf.json
│   ├── tauri.ios.conf.json
│   └── tauri.android.conf.json
└── docs/
```

---

## 🧪 Testing

```sh
# Run all Rust unit tests
cd src-tauri
cargo test

# Run with output
cargo test -- --nocapture

# Run a specific test
cargo test script_contains_invoke
```

### Current test coverage

| Module | Tests | What they verify |
|---|---|---|
| `adblock.rs` | 2 | `adblock_script()` contains `check_url_blocked` and `get_page_cosmetic_filters`; bundled rules file exists and contains expected content |

---

## 🔨 Making changes

### Adding a new feature module

1. Create `src-tauri/src/ext/your_feature.rs`
2. Add `pub mod your_feature;` to `src-tauri/src/ext/mod.rs`
3. Implement Tauri commands with `#[tauri::command]`
4. Register commands in `lib.rs::run()` via `generate_handler![]`
5. Add any setup logic in the `setup()` closure
6. Update documentation in `docs/`

### Modifying adblock rules

1. Edit `src-tauri/src/ext/adblock_bundled.txt`
2. Add filter rules in Adblock Plus / uBlock Origin syntax
3. For URLs: `||domain.com^$third-party`
4. For cosmetic hiding: `site.com##.ad-class` or `site.com##a[href*="ad-link"]`
5. Rebuild — the file is compiled into binary via `include_str!`

### Changing the JS injection script

1. Edit the `adblock_script()` function in `src-tauri/src/ext/adblock.rs`
2. The script is injected via `.initialization_script()` and runs before page content
3. Tests check that `check_url_blocked` and `get_page_cosmetic_filters` strings exist in the output
4. Keep the script minimal — heavy interception causes app freezing

---

## 🏗️ Building platform-specific releases

### Windows

```sh
cargo tauri build
# Output: target/release/rule34video.msi, rule34video Setup.exe
```

### macOS

```sh
cargo tauri build --target aarch64-apple-darwin
# Output: target/aarch64-apple-darwin/release/bundle/
```

### Linux

```sh
cargo tauri build --target x86_64-unknown-linux-gnu
# Output: target/x86_64-unknown-linux-gnu/release/bundle/
```

### Android

```sh
cargo tauri build --target aarch64-linux-android
# Requires: Android SDK + NDK configured in .cargo/config.toml
```

### iOS

```sh
cargo tauri build --target aarch64-apple-ios
# Requires: Xcode 15+, CocoaPods
```

---

## 🐛 Debugging

### Enable logging

The app uses `println!` for debug output (visible in the terminal running `cargo tauri dev`). Key debug points:

- Filter download progress
- Navigation URL checks
- Download events
- Child window open/close
- Context menu requests

### Adblock debugging

1. **Check if the engine has loaded**: Look for "adblock" messages in the terminal
2. **Test a specific URL**: Add a temporary `println!` in `is_blocked()` to log checked URLs
3. **Verify JS injection**: Open DevTools (if available) and check for the `__ab__` style element
4. **Add filter rules**: The fastest way to test is adding rules to `adblock_bundled.txt`

### Known pitfalls

| Pitfall | Solution |
|---|---|
| WebView2 COM calls fail if webview isn't initialized | Always handle `Result` errors, never `.unwrap()` |
| `Engine` is `!Send + !Sync` | Wrap in `Mutex<Engine>` with `unsafe impl Send+Sync` |
| JS injection freezes the app | Keep it minimal — no `setAttribute` override, no `querySelectorAll` in MutationObserver, no 5s `setInterval` |
| Engine empty on first launch | Bundled rules via `include_str!` ensure non-empty engine |
| Async filter download completes after page loads | Engine starts with bundled rules, gets replaced atomically |

---

## 📦 Release process

1. Update version in `Cargo.toml`, `tauri.conf.json`, `AGENTS.md`, `README.md`, `CHANGELOG.md`
2. Run `cargo test` to verify everything passes
3. Run `cargo build` to verify compilation
4. Commit with Conventional Commits format
5. Tag the release
6. Build platform-specific packages via `cargo tauri build`
7. Publish GitHub release
