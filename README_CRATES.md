# tui-banner

Cinematic ANSI banners for Rust CLI/TUI.

## Features

- Grid-first rendering pipeline
- Bundled DOS Rebel (Figlet) font + load any `.flf`
- Truecolor / 256-color / no-color output with auto-detect
- Gradients, pixel fill, dithering, shadows, edge shading
- Named style and palette presets
- Fluent builder API

## Quick Start

```toml
[dependencies]
tui-banner = { path = "." }
```

```rust
use tui_banner::{Align, Banner, Style};

fn main() -> Result<(), tui_banner::BannerError> {
    let banner = Banner::new("RUST CLI")?
        .style(Style::NeonCyber)
        .render();

    println!("{banner}");
    Ok(())
}
```
