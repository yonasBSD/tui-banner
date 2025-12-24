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

use std::io::{self, Write};
use std::thread;
use std::time::Duration;

use crate::color::Palette;
use crate::color::{Color, ColorMode};
use crate::effects::dither::apply_dot_dither;
use crate::effects::light_sweep::{LightSweep, SweepDirection, apply_light_sweep_tint};
use crate::effects::outline::{EdgeShade, apply_edge_shade};
use crate::effects::shadow::{Shadow, apply_shadow};
use crate::emit::emit_ansi;
use crate::fill::{Dither, Fill, apply_fill};
use crate::font::{self, Font, render_text};
use crate::frame::{Frame, apply_frame};
use crate::gradient::Gradient;
use crate::grid::{Align, Grid, Padding};
use crate::style::Style;
use crate::terminal::detect_color_mode;

/// High-level banner builder.
#[derive(Clone, Debug)]
pub struct Banner {
    text: String,
    font: Font,
    gradient: Option<Gradient>,
    fill: Fill,
    light_sweep: Option<LightSweep>,
    shadow: Option<Shadow>,
    edge_shade: Option<EdgeShade>,
    dot_dither: Option<Dither>,
    dot_dither_targets: Option<Vec<char>>,
    align: Align,
    padding: Padding,
    frame: Option<Frame>,
    width: Option<usize>,
    max_width: Option<usize>,
    kerning: usize,
    line_gap: usize,
    trim_vertical: bool,
    color_mode: ColorMode,
}

/// Errors returned when building a banner.
#[derive(Debug)]
pub enum BannerError {
    /// Failed to parse the bundled Figlet font.
    Font(font::figlet::FigletError),
}

impl std::fmt::Display for BannerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BannerError::Font(err) => write!(f, "font parse error: {err:?}"),
        }
    }
}

impl std::error::Error for BannerError {}

impl From<font::figlet::FigletError> for BannerError {
    fn from(err: font::figlet::FigletError) -> Self {
        BannerError::Font(err)
    }
}

impl Banner {
    /// Create a banner from text.
    ///
    /// Returns an error if the bundled font cannot be parsed.
    pub fn new(text: impl Into<String>) -> Result<Self, BannerError> {
        Ok(Self {
            text: text.into(),
            font: Font::dos_rebel()?,
            gradient: None,
            fill: Fill::Blocks,
            light_sweep: None,
            shadow: None,
            edge_shade: None,
            dot_dither: None,
            dot_dither_targets: None,
            align: Align::Left,
            padding: Padding::uniform(0),
            frame: None,
            width: None,
            max_width: None,
            kerning: 1,
            line_gap: 0,
            trim_vertical: false,
            color_mode: ColorMode::Auto,
        })
    }

    /// Set the font.
    pub fn font(mut self, font: Font) -> Self {
        self.font = font;
        self
    }

    /// Apply a named style preset.
    pub fn style(mut self, style: Style) -> Self {
        self.color_mode = ColorMode::TrueColor;
        self.gradient = Some(Gradient::vertical(Palette::preset(style.preset())));
        self.fill = Fill::Keep;
        self
    }

    /// Apply a gradient across the glyph grid.
    pub fn gradient(mut self, gradient: Gradient) -> Self {
        self.gradient = Some(gradient);
        self
    }

    /// Fill visible cells (or keep glyph characters).
    pub fn fill(mut self, fill: Fill) -> Self {
        self.fill = fill;
        self
    }

    /// Add a drop shadow.
    pub fn shadow(mut self, offset: (i32, i32), alpha: f32) -> Self {
        self.shadow = Some(Shadow { offset, alpha });
        self
    }

    /// Add a highlight sweep (useful for animated passes).
    pub fn light_sweep(mut self, sweep: LightSweep) -> Self {
        self.light_sweep = Some(sweep);
        self
    }

    /// Add a 1-cell edge shade using a darker color and a dedicated character.
    pub fn edge_shade(mut self, darken: f32, ch: char) -> Self {
        self.edge_shade = Some(EdgeShade { ch, darken });
        self
    }

    /// Enable dot dithering using a custom configuration.
    pub fn dot_dither(mut self, dither: Dither) -> Self {
        self.dot_dither = Some(dither);
        self
    }

    /// Set the dither targets (glyphs to be replaced by dots).
    pub fn dot_dither_targets(mut self, targets: &[char]) -> Self {
        self.dot_dither_targets = Some(targets.to_vec());
        self
    }

    /// Set the dither targets using a string (e.g. "░▒▓").
    pub fn dot_dither_targets_str(mut self, targets: &str) -> Self {
        self.dot_dither_targets = Some(targets.chars().collect());
        self
    }

    /// Builder-style dot dithering configuration.
    pub fn dither(self) -> DotDitherBuilder {
        DotDitherBuilder::new(self)
    }

    /// Align within the target width.
    pub fn align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }

    /// Add padding around the banner.
    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }

    /// Add a frame around the banner.
    pub fn frame(mut self, frame: Frame) -> Self {
        self.frame = Some(frame);
        self
    }

    /// Force an output width (pads or clips).
    pub fn width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }

    /// Clamp output width.
    pub fn max_width(mut self, width: usize) -> Self {
        self.max_width = Some(width);
        self
    }

    /// Space between characters.
    pub fn kerning(mut self, kerning: usize) -> Self {
        self.kerning = kerning;
        self
    }

    /// Blank lines between text lines.
    pub fn line_gap(mut self, line_gap: usize) -> Self {
        self.line_gap = line_gap;
        self
    }

    /// Trim blank rows from the top and bottom of the rendered grid.
    pub fn trim_vertical(mut self, enabled: bool) -> Self {
        self.trim_vertical = enabled;
        self
    }

    /// Override color mode.
    pub fn color_mode(mut self, mode: ColorMode) -> Self {
        self.color_mode = mode;
        self
    }

    /// Render to a `String` (ANSI escapes included if enabled).
    pub fn render(&self) -> String {
        self.render_with_sweep(None, None)
    }

    /// Animate a light sweep over the banner.
    ///
    /// `speed_ms` controls the delay between frames in milliseconds.
    /// `highlight` overrides the sweep color (use `None` for white).
    pub fn animate_sweep(&self, speed_ms: u64, highlight: Option<Color>) -> io::Result<()> {
        let mut stdout = io::stdout();
        write!(stdout, "\x1b[2J\x1b[?25l")?;
        stdout.flush()?;

        let frames = 180;
        let frame_time = Duration::from_millis(speed_ms);
        let highlight = highlight.unwrap_or(Color::Rgb(255, 255, 255));
        let base = self.light_sweep.unwrap_or_else(|| {
            LightSweep::new(SweepDirection::DiagonalDown)
                .width(0.25)
                .intensity(0.9)
                .softness(2.5)
        });
        let start = base.center - 0.75;
        let end = base.center + 0.75;
        for frame in 0..frames {
            let t = frame as f32 / frames as f32;
            let center = start + t * (end - start);
            let sweep = base.center(center);

            let banner = self.render_with_sweep(Some(sweep), Some(highlight));
            write!(stdout, "\x1b[H{banner}")?;
            stdout.flush()?;
            thread::sleep(frame_time);
        }

        writeln!(stdout, "\x1b[?25h")?;
        Ok(())
    }

    /// Animate a wave-like breathing effect over the banner without moving glyphs.
    ///
    /// `speed_ms` controls the delay between frames in milliseconds.
    /// `dim_strength` and `bright_strength` tune the low/high brightness (defaults are used when `None`).
    pub fn animate_wave(
        &self,
        speed_ms: u64,
        dim_strength: Option<f32>,
        bright_strength: Option<f32>,
    ) -> io::Result<()> {
        let mut stdout = io::stdout();
        write!(stdout, "\x1b[2J\x1b[?25l")?;
        stdout.flush()?;

        let frames = 180;
        let frame_time = Duration::from_millis(speed_ms);
        let base = self.render_grid_with_sweep(None, None);
        let dim_strength = dim_strength.unwrap_or(0.35).clamp(0.0, 1.0);
        let bright_strength = bright_strength.unwrap_or(0.2).clamp(0.0, 1.0);
        let mode = match self.color_mode {
            ColorMode::Auto => detect_color_mode(),
            other => other,
        };

        for frame in 0..frames {
            let t = frame as f32 / frames as f32;
            let phase = t * std::f32::consts::TAU;
            let waved = apply_wave_breathe(&base, phase, dim_strength, bright_strength);
            let banner = emit_ansi(&waved, mode);
            write!(stdout, "\x1b[H{banner}")?;
            stdout.flush()?;
            thread::sleep(frame_time);
        }

        writeln!(stdout, "\x1b[?25h")?;
        Ok(())
    }

    /// Animate a rolling wave (tsunami roll) that advances with a heavy crest.
    ///
    /// `speed_ms` controls the delay between frames in milliseconds.
    pub fn animate_roll(&self, speed_ms: u64) -> io::Result<()> {
        let mut stdout = io::stdout();
        write!(stdout, "\x1b[2J\x1b[?25l")?;
        stdout.flush()?;

        let frames = 180;
        let frame_time = Duration::from_millis(speed_ms);
        let base = self.render_grid_with_sweep(None, None);
        let mode = match self.color_mode {
            ColorMode::Auto => detect_color_mode(),
            other => other,
        };

        for frame in 0..frames {
            let t = frame as f32 / frames as f32;
            let rolled = apply_roll(&base, t);
            let banner = emit_ansi(&rolled, mode);
            write!(stdout, "\x1b[H{banner}")?;
            stdout.flush()?;
            thread::sleep(frame_time);
        }

        writeln!(stdout, "\x1b[?25h")?;
        Ok(())
    }

    fn render_with_sweep(
        &self,
        sweep_override: Option<LightSweep>,
        highlight: Option<Color>,
    ) -> String {
        let grid = self.render_grid_with_sweep(sweep_override, highlight);
        let mode = match self.color_mode {
            ColorMode::Auto => detect_color_mode(),
            other => other,
        };
        emit_ansi(&grid, mode)
    }

    fn render_grid_with_sweep(
        &self,
        sweep_override: Option<LightSweep>,
        highlight: Option<Color>,
    ) -> Grid {
        let mut grid = render_text(&self.text, &self.font, self.kerning, self.line_gap);
        apply_fill(&mut grid, self.fill);
        if let Some(gradient) = &self.gradient {
            gradient.apply(&mut grid);
        }
        if let Some(sweep) = sweep_override.or(self.light_sweep) {
            let highlight = highlight.unwrap_or(Color::Rgb(255, 255, 255));
            apply_light_sweep_tint(&mut grid, sweep, highlight);
        }
        if let Some(dither) = self.dot_dither {
            let default_targets = ['░', '▒'];
            let targets = self
                .dot_dither_targets
                .as_deref()
                .unwrap_or(&default_targets);
            grid = apply_dot_dither(&grid, dither, targets);
        }
        if let Some(shade) = self.edge_shade {
            grid = apply_edge_shade(&grid, shade);
        }
        if let Some(shadow) = self.shadow {
            grid = apply_shadow(&grid, shadow);
        }
        if self.trim_vertical {
            grid = grid.trim_vertical();
        }
        let grid = apply_layout(grid, self.padding, self.width, self.max_width, self.align);
        if let Some(frame) = &self.frame {
            apply_frame(grid, frame)
        } else {
            grid
        }
    }
}

/// Builder for dot dithering over selected glyph targets.
pub struct DotDitherBuilder {
    banner: Banner,
    targets: Vec<char>,
    dots: (char, char),
}

impl DotDitherBuilder {
    fn new(banner: Banner) -> Self {
        Self {
            banner,
            targets: vec!['░', '▒'],
            dots: ('░', '░'),
        }
    }

    /// Set glyphs to be replaced by dots.
    pub fn targets(mut self, targets: &str) -> Self {
        self.targets = targets.chars().collect();
        self
    }

    /// Set glyphs to be replaced by dots.
    pub fn targets_vec(mut self, targets: &[char]) -> Self {
        self.targets = targets.to_vec();
        self
    }

    /// Set dot characters (1 or 2 chars, e.g. "·:").
    pub fn dots(mut self, dots: &str) -> Self {
        self.dots = parse_dots(dots);
        self
    }

    /// Apply a checkerboard-style dither.
    pub fn checker(mut self, period: u8) -> Banner {
        let dither = Dither {
            mode: crate::fill::DitherMode::Checker { period },
            dot: self.dots.0,
            alt: self.dots.1,
        };
        self.banner = self
            .banner
            .dot_dither(dither)
            .dot_dither_targets(&self.targets);
        self.banner
    }

    /// Apply a hash-noise dither.
    pub fn noise(mut self, seed: u32, threshold: u8) -> Banner {
        let dither = Dither {
            mode: crate::fill::DitherMode::Noise { seed, threshold },
            dot: self.dots.0,
            alt: self.dots.1,
        };
        self.banner = self
            .banner
            .dot_dither(dither)
            .dot_dither_targets(&self.targets);
        self.banner
    }
}

fn parse_dots(dots: &str) -> (char, char) {
    let mut iter = dots.chars();
    let first = iter.next().unwrap_or('·');
    let second = iter.next().unwrap_or(first);
    (first, second)
}

fn apply_layout(
    mut grid: Grid,
    padding: Padding,
    width: Option<usize>,
    max_width: Option<usize>,
    align: Align,
) -> Grid {
    let height = grid.height();
    let width_now = grid.width();
    let padded_width = width_now + padding.left + padding.right;
    let padded_height = height + padding.top + padding.bottom;

    let mut padded = Grid::new(padded_height, padded_width);
    padded.blit(&grid, padding.top, padding.left);
    grid = padded;

    let mut target_width = width;
    if let Some(max_width) = max_width {
        let limit = grid.width().min(max_width);
        target_width = Some(target_width.map_or(limit, |w| w.min(max_width)));
    }

    if let Some(target) = target_width {
        if target > grid.width() {
            let extra = target - grid.width();
            let left_extra = match align {
                Align::Left => 0,
                Align::Center => extra / 2,
                Align::Right => extra,
            };
            let right_extra = extra - left_extra;
            let mut expanded = Grid::new(grid.height(), target);
            expanded.blit(&grid, 0, left_extra);
            if right_extra > 0 {
                // already blank by default
            }
            grid = expanded;
        } else if target < grid.width() {
            grid = clip_width(&grid, target, align);
        }
    }

    grid
}

fn clip_width(grid: &Grid, target: usize, align: Align) -> Grid {
    if target == 0 {
        return Grid::new(grid.height(), 0);
    }

    let start = match align {
        Align::Left => 0,
        Align::Center => (grid.width().saturating_sub(target)) / 2,
        Align::Right => grid.width().saturating_sub(target),
    };

    let mut out = Grid::new(grid.height(), target);
    for r in 0..grid.height() {
        for c in 0..target {
            if let (Some(cell), Some(target_cell)) = (grid.cell(r, start + c), out.cell_mut(r, c)) {
                *target_cell = cell.clone();
            }
        }
    }
    out
}

fn apply_wave_breathe(grid: &Grid, phase: f32, dim_strength: f32, bright_strength: f32) -> Grid {
    let height = grid.height();
    let width = grid.width();
    if height == 0 || width == 0 {
        return grid.clone();
    }

    let mut out = grid.clone();

    for row in 0..height {
        for col in 0..width {
            let wave = scale_wave(phase, row, col, width, height);
            let (dim, bright) = if wave < 0.5 {
                let t = (0.5 - wave) / 0.5;
                (dim_strength * t, 0.0)
            } else {
                let t = (wave - 0.5) / 0.5;
                (0.0, bright_strength * t)
            };
            let Some(cell) = out.cell_mut(row, col) else {
                continue;
            };
            if !cell.visible {
                continue;
            }
            if let Some(color) = cell.fg {
                cell.fg = Some(apply_breathe_color(color, dim, bright));
            }
        }
    }

    out
}

fn apply_roll(grid: &Grid, t: f32) -> Grid {
    let height = grid.height();
    let width = grid.width();
    if height == 0 || width == 0 {
        return grid.clone();
    }

    let center = -0.2 + t * 1.4;
    let front_width = 0.06;
    let back_width = 0.22;
    let bright_strength = 0.6;
    let dim_strength = 0.5;
    let mid = (height as f32 - 1.0) / 2.0;

    let mut out = Grid::new(height, width);
    for row in 0..height {
        let row_falloff = if height > 1 {
            let rel = ((row as f32 - mid).abs() / mid).min(1.0);
            1.0 - 0.25 * rel
        } else {
            1.0
        };
        for col in 0..width {
            let Some(source) = grid.cell(row, col) else {
                continue;
            };
            if !source.visible {
                continue;
            }

            let x = if width > 1 {
                col as f32 / (width - 1) as f32
            } else {
                0.0
            };
            let d = x - center;
            let mut base_color = source.fg.unwrap_or(Color::Rgb(255, 255, 255));
            if d > 0.0 {
                base_color = Color::Rgb(255, 255, 255);
            }
            let mut bright = 0.0;
            let mut dim = 0.0;

            if d >= 0.0 && d <= front_width {
                let t = 1.0 - d / front_width;
                bright = t.powf(1.7);
            } else if d < 0.0 && d >= -back_width {
                let t = 1.0 - (-d) / back_width;
                dim = t.powf(1.2);
            }

            let crest = if d >= 0.0 && d <= front_width {
                let t = 1.0 - d / front_width;
                t.powf(1.4)
            } else {
                0.0
            };
            let offset = -(crest * 1.0).round() as i32;

            let bright_amt = (bright * bright_strength * row_falloff).clamp(0.0, 1.0);
            let dim_amt = (dim * dim_strength * row_falloff).clamp(0.0, 1.0);

            let dest = row as i32 + offset;
            if dest < 0 || dest >= height as i32 {
                continue;
            }

            let mut cell = source.clone();
            cell.fg = Some(apply_breathe_color(base_color, dim_amt, bright_amt));
            if let Some(target) = out.cell_mut(dest as usize, col) {
                *target = cell;
            }
        }
    }

    out
}

fn scale_wave(phase: f32, row: usize, col: usize, width: usize, height: usize) -> f32 {
    let fx = if width > 1 {
        col as f32 / (width - 1) as f32
    } else {
        0.0
    };
    let fy = if height > 1 {
        row as f32 / (height - 1) as f32
    } else {
        0.0
    };

    let freq_x = 5.0;
    let freq_y = 3.0;
    let phase_offset = (fx * freq_x + fy * freq_y) * std::f32::consts::TAU;
    ((phase + phase_offset).sin() + 1.0) * 0.5
}

fn apply_breathe_color(color: Color, dim: f32, bright: f32) -> Color {
    let dimmed = if dim > 0.0 {
        color.lerp(Color::Rgb(0, 0, 0), dim.clamp(0.0, 1.0))
    } else {
        color
    };
    if bright > 0.0 {
        dimmed.lerp(Color::Rgb(255, 255, 255), bright.clamp(0.0, 1.0))
    } else {
        dimmed
    }
}
