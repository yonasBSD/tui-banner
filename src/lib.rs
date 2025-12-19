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

#![deny(missing_docs)]
//! Colorful ASCII art banner rendering for Rust CLI/TUI.
//!
//! ## Quick Start
//! ```rust
//! use tui_banner::{Align, Banner, Style};
//!
//! # fn main() -> Result<(), tui_banner::BannerError> {
//!
//! let banner = Banner::new("RUST CLI")?
//!     .style(Style::NeonCyber)
//!     .align(Align::Center)
//!     .padding(1)
//!     .render();
//!
//! let _ = banner;
//! # Ok(())
//! # }
//! ```

/// High-level banner builder API.
pub mod banner;
/// Color types and palettes.
pub mod color;
/// Visual effects (dither, outline, shadow).
pub mod effects;
/// ANSI output emitter.
pub mod emit;
/// Fill and dither configuration.
pub mod fill;
/// Fonts and glyph rendering.
pub mod font;
/// Gradient definitions.
pub mod gradient;
/// Grid and layout types.
pub mod grid;
/// Named banner styles.
pub mod style;
/// Terminal capability detection.
pub mod terminal;

pub use banner::{Banner, BannerError};
pub use color::{Color, ColorMode, Palette, Preset};
pub use effects::outline::EdgeShade;
pub use fill::{Dither, DitherMode, Fill};
pub use font::{Font, figlet::FigletError};
pub use gradient::{Gradient, GradientDirection};
pub use grid::{Align, Padding};
pub use style::Style;
