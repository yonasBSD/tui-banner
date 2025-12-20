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

use tui_banner::emit::emit_ansi;
use tui_banner::fill::apply_fill;
use tui_banner::font::{Font, render_text};
use tui_banner::{ColorMode, Fill, Gradient, Palette};

fn main() -> Result<(), tui_banner::BannerError> {
    println!();
    let font = Font::dos_rebel()?;
    let mut grid = render_text("RUST CLI", &font, 1, 0);
    apply_fill(&mut grid, Fill::Keep);

    let gradient = Gradient::horizontal(Palette::from_hex(&[
        "#FFE29A", // warm light
        "#FF8C42", // orange
        "#FF3D7F", // pink
    ]));
    gradient.apply(&mut grid);

    let banner = emit_ansi(&grid, ColorMode::TrueColor);
    println!("{banner}");
    Ok(())
}
