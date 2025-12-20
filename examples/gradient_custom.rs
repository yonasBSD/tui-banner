// Copyright (c) 2025 Lei Zhang
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.

use tui_banner::{Align, Banner, ColorMode, Fill, Gradient, Palette};

fn main() -> Result<(), tui_banner::BannerError> {
    println!();
    let banner = Banner::new("RUST CLI")? // text
        .color_mode(ColorMode::TrueColor) // true color
        .gradient(Gradient::vertical(Palette::from_hex(&[
            "#00E5FF", // cyan
            "#3A7BFF", // blue
            "#E6F6FF", // ice
        ]))) // gradient stops
        .fill(Fill::Keep) // keep glyphs
        .dither()
        .targets("░▒▓") // dither targets
        .checker(3) // checker period
        .align(Align::Center) // center align
        .padding(1) // uniform padding
        .render();

    println!("{banner}");
    Ok(())
}
