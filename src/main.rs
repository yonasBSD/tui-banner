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

use tui_banner::{Align, Banner, Fill, Gradient, Palette};

fn main() {
    let banner = match Banner::new("RUST CLI") {
        Ok(banner) => banner
            .gradient(Gradient::vertical(Palette::from_hex(&[
                "#00E5FF", "#7B5CFF", "#FF5AD9",
            ])))
            .fill(Fill::Keep)
            .dither()
            .targets("░▒▓")
            .checker(3)
            .align(Align::Center)
            .padding(1)
            .render(),
        Err(err) => {
            eprintln!("tui-banner: {err}");
            String::new()
        }
    };

    println!("{banner}");
}
