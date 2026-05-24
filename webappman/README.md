# WebAppMan

Small terminal UI to manage and launch web apps as "app windows" using a system browser.

**Overview**
- TUI built with Rust using `crossterm` + `ratatui`.
- Stores apps in `$HOME/.config/webappman/apps.txt` as `Name|URL` lines.
- Uses helper scripts in `../scripts` (or `./scripts`) to add/remove/launch apps.

**Requirements**
- Linux (tested), Rust toolchain (cargo), and a Chromium-compatible browser available in PATH (`chromium`, `google-chrome`, etc.).

**Build**
```bash
cd webappman
cargo build --release
```

**Run**
```bash
# run the built binary
./target/release/webappman
# or for debug builds
./target/debug/webappman
```

**Keyboard controls**
- Enter: Launch the selected app (runs scripts/launch.sh)
- a: Toggle "All Apps" view (shows `name | url`)
- r: Reload apps from disk
- d: Remove the selected app (runs scripts/remove.sh)
- q: Quit

**Scripts & storage**
- Scripts are located in the repository `scripts/` folder (`add.sh`, `remove.sh`, `launch.sh`). The app tries to find the scripts relative to the binary and project root.
- Apps file: `$HOME/.config/webappman/apps.txt` — each line: `Name|https://...`.

**Add an app**
- From shell:
```bash
./scripts/add.sh "MyApp" "https://example.com"
```
- In the TUI: add support can be added (interactive prompt). Press `r` after adding to reload the list.

**Troubleshooting**
- If launching prints `No supported browser found`, ensure `chromium` or a compatible browser is installed and on PATH.
- Make sure scripts are executable: `chmod +x scripts/*.sh`.
- If the TUI cannot find `scripts`, run the binary from the `webappman` folder or ensure the `scripts` folder is alongside the binary or parent directories.

**Development notes**
- `src/bash.rs` now searches for `scripts/` in a few locations and falls back to `scripts/<name>`.
- `src/ui.rs` and `src/input.rs` implement the basic list, full view, and key handling.

.
