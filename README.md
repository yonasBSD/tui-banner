# tui-banner

Cinematic ANSI banners for Rust CLI/TUI.

![banner](assets/banner.png)

## Features

- Grid-first rendering pipeline
- Bundled DOS Rebel (Figlet) font + load any `.flf`
- Truecolor / 256-color / no-color output with auto-detect
- Gradients, pixel fill, dithering, shadows, edge shading
- Fluent builder API

## Quick Start

```toml
[dependencies]
tui-banner = { path = "." }
```

```rust
use tui_banner::{Align, Banner, Fill, Gradient, Palette};

fn main() -> Result<(), tui_banner::BannerError> {
    let banner = Banner::new("RUST CLI")?
        .gradient(Gradient::vertical(Palette::from_hex(&[
            "#00E5FF",
            "#7B5CFF",
            "#FF5AD9",
        ])))
        .fill(Fill::Keep)
        .dither()
        .targets("░▒▓")
        .dots("·:")
        .checker(3)
        .align(Align::Center)
        .padding(1)
        .render();

    println!("{banner}");
    Ok(())
}
```

## Examples

```bash
cargo run --example basic
cargo run --example effects
cargo run --example multiline
cargo run --example no_color
cargo run --example pixel
```
