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

use crate::color::Color;
use crate::gradient::Gradient;
use crate::grid::Grid;

/// Predefined frame styles.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FrameStyle {
    /// Single-line box drawing.
    Single,
    /// Double-line box drawing.
    Double,
    /// Rounded corners.
    Rounded,
    /// Heavy stroke.
    Heavy,
    /// ASCII fallback using +-| characters.
    Ascii,
}

/// Character set for rendering frames.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FrameChars {
    /// Top-left corner.
    pub top_left: char,
    /// Top-right corner.
    pub top_right: char,
    /// Bottom-left corner.
    pub bottom_left: char,
    /// Bottom-right corner.
    pub bottom_right: char,
    /// Horizontal line.
    pub horizontal: char,
    /// Vertical line.
    pub vertical: char,
}

/// Color treatment for frame strokes.
#[derive(Clone, Debug)]
pub enum FramePaint {
    /// Solid color for the entire frame.
    Solid(Color),
    /// Gradient across the frame bounds.
    Gradient(Gradient),
}

/// Frame configuration (glyphs + optional color treatment).
#[derive(Clone, Debug)]
pub struct Frame {
    chars: FrameChars,
    paint: Option<FramePaint>,
}

impl FrameStyle {
    /// Resolve the glyph set for this style.
    pub fn chars(self) -> FrameChars {
        match self {
            FrameStyle::Single => FrameChars::new('┌', '┐', '└', '┘', '─', '│'),
            FrameStyle::Double => FrameChars::new('╔', '╗', '╚', '╝', '═', '║'),
            FrameStyle::Rounded => FrameChars::new('╭', '╮', '╰', '╯', '─', '│'),
            FrameStyle::Heavy => FrameChars::new('┏', '┓', '┗', '┛', '━', '┃'),
            FrameStyle::Ascii => FrameChars::new('+', '+', '+', '+', '-', '|'),
        }
    }
}

impl FrameChars {
    /// Build a custom frame character set.
    pub const fn new(
        top_left: char,
        top_right: char,
        bottom_left: char,
        bottom_right: char,
        horizontal: char,
        vertical: char,
    ) -> Self {
        Self {
            top_left,
            top_right,
            bottom_left,
            bottom_right,
            horizontal,
            vertical,
        }
    }
}

impl Frame {
    /// Create a frame from a built-in style.
    pub fn new(style: FrameStyle) -> Self {
        Self {
            chars: style.chars(),
            paint: None,
        }
    }

    /// Create a frame from a custom character set.
    pub fn custom(chars: FrameChars) -> Self {
        Self { chars, paint: None }
    }

    /// Apply a solid color to the frame.
    pub fn color(mut self, color: Color) -> Self {
        self.paint = Some(FramePaint::Solid(color));
        self
    }

    /// Apply a gradient to the frame.
    pub fn gradient(mut self, gradient: Gradient) -> Self {
        self.paint = Some(FramePaint::Gradient(gradient));
        self
    }

    pub(crate) fn chars(&self) -> FrameChars {
        self.chars
    }

    pub(crate) fn paint(&self) -> Option<&FramePaint> {
        self.paint.as_ref()
    }
}

pub(crate) fn apply_frame(grid: Grid, frame: &Frame) -> Grid {
    let inner_height = grid.height();
    let inner_width = grid.width();
    let out_height = inner_height + 2;
    let out_width = inner_width + 2;
    let mut framed = Grid::new(out_height, out_width);
    let chars = frame.chars();

    set_cell(&mut framed, 0, 0, chars.top_left);
    set_cell(&mut framed, 0, out_width - 1, chars.top_right);
    set_cell(&mut framed, out_height - 1, 0, chars.bottom_left);
    set_cell(
        &mut framed,
        out_height - 1,
        out_width - 1,
        chars.bottom_right,
    );

    if out_width > 2 {
        for col in 1..out_width - 1 {
            set_cell(&mut framed, 0, col, chars.horizontal);
            set_cell(&mut framed, out_height - 1, col, chars.horizontal);
        }
    }

    if out_height > 2 {
        for row in 1..out_height - 1 {
            set_cell(&mut framed, row, 0, chars.vertical);
            set_cell(&mut framed, row, out_width - 1, chars.vertical);
        }
    }

    if let Some(paint) = frame.paint() {
        match paint {
            FramePaint::Solid(color) => {
                apply_solid_color(&mut framed, *color);
            }
            FramePaint::Gradient(gradient) => {
                gradient.apply(&mut framed);
            }
        }
    }

    framed.blit(&grid, 1, 1);
    framed
}

fn set_cell(grid: &mut Grid, row: usize, col: usize, ch: char) {
    if let Some(cell) = grid.cell_mut(row, col) {
        cell.ch = ch;
        cell.visible = ch != ' ';
    }
}

fn apply_solid_color(grid: &mut Grid, color: Color) {
    for row in grid.rows_mut() {
        for cell in row {
            if cell.visible {
                cell.fg = Some(color);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::Grid;

    #[test]
    fn wraps_grid_with_frame() {
        let grid = Grid::from_char_rows(vec![vec!['A']]);
        let frame = Frame::new(FrameStyle::Single);
        let framed = apply_frame(grid, &frame);

        assert_eq!(framed.height(), 3);
        assert_eq!(framed.width(), 3);
        assert_eq!(framed.cell(0, 0).unwrap().ch, '┌');
        assert_eq!(framed.cell(0, 2).unwrap().ch, '┐');
        assert_eq!(framed.cell(2, 0).unwrap().ch, '└');
        assert_eq!(framed.cell(2, 2).unwrap().ch, '┘');
        assert_eq!(framed.cell(1, 1).unwrap().ch, 'A');
    }
}
