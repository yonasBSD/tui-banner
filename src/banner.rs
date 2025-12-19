use crate::color::ColorMode;
use crate::emit::emit_ansi;
use crate::effects::outline::{apply_edge_shade, EdgeShade};
use crate::effects::shadow::{apply_shadow, Shadow};
use crate::fill::{apply_fill, Fill};
use crate::font::{render_text, Font};
use crate::gradient::Gradient;
use crate::grid::{Align, Grid, Padding};
use crate::terminal::detect_color_mode;

#[derive(Clone, Debug)]
pub struct Banner {
    text: String,
    font: Font,
    gradient: Option<Gradient>,
    fill: Fill,
    shadow: Option<Shadow>,
    edge_shade: Option<EdgeShade>,
    align: Align,
    padding: Padding,
    width: Option<usize>,
    max_width: Option<usize>,
    kerning: usize,
    line_gap: usize,
    color_mode: ColorMode,
}

impl Banner {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            font: Font::block(),
            gradient: None,
            fill: Fill::Blocks,
            shadow: None,
            edge_shade: None,
            align: Align::Left,
            padding: Padding::uniform(0),
            width: None,
            max_width: None,
            kerning: 1,
            line_gap: 0,
            color_mode: ColorMode::Auto,
        }
    }

    pub fn font(mut self, font: Font) -> Self {
        self.font = font;
        self
    }

    pub fn gradient(mut self, gradient: Gradient) -> Self {
        self.gradient = Some(gradient);
        self
    }

    pub fn fill(mut self, fill: Fill) -> Self {
        self.fill = fill;
        self
    }

    pub fn shadow(mut self, offset: (i32, i32), alpha: f32) -> Self {
        self.shadow = Some(Shadow { offset, alpha });
        self
    }

    pub fn edge_shade(mut self, darken: f32, ch: char) -> Self {
        self.edge_shade = Some(EdgeShade { ch, darken });
        self
    }

    pub fn align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }

    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }

    pub fn width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }

    pub fn max_width(mut self, width: usize) -> Self {
        self.max_width = Some(width);
        self
    }

    pub fn kerning(mut self, kerning: usize) -> Self {
        self.kerning = kerning;
        self
    }

    pub fn line_gap(mut self, line_gap: usize) -> Self {
        self.line_gap = line_gap;
        self
    }

    pub fn color_mode(mut self, mode: ColorMode) -> Self {
        self.color_mode = mode;
        self
    }

    pub fn render(&self) -> String {
        let mut grid = render_text(&self.text, &self.font, self.kerning, self.line_gap);
        apply_fill(&mut grid, self.fill);
        if let Some(gradient) = &self.gradient {
            gradient.apply(&mut grid);
        }
        if let Some(shade) = self.edge_shade {
            grid = apply_edge_shade(&grid, shade);
        }
        if let Some(shadow) = self.shadow {
            grid = apply_shadow(&grid, shadow);
        }
        let grid = apply_layout(grid, self.padding, self.width, self.max_width, self.align);
        let mode = match self.color_mode {
            ColorMode::Auto => detect_color_mode(),
            other => other,
        };
        emit_ansi(&grid, mode)
    }
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
            if let Some(cell) = grid.cell(r, start + c) {
                if let Some(target_cell) = out.cell_mut(r, c) {
                    *target_cell = cell.clone();
                }
            }
        }
    }
    out
}
