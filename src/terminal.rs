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

use std::env;

use crate::color::ColorMode;

/// Detect terminal color capability.
pub fn detect_color_mode() -> ColorMode {
    if env::var("NO_COLOR").is_ok() {
        return ColorMode::NoColor;
    }

    let colorterm = env::var("COLORTERM").unwrap_or_default().to_lowercase();
    if colorterm.contains("truecolor") || colorterm.contains("24bit") {
        return ColorMode::TrueColor;
    }

    let term = env::var("TERM").unwrap_or_default().to_lowercase();
    if term.contains("256color") {
        return ColorMode::Ansi256;
    }

    ColorMode::NoColor
}
