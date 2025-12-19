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
use crate::grid::Grid;

/// Edge shading configuration.
#[derive(Clone, Copy, Debug)]
pub struct EdgeShade {
    /// Character used for the edge.
    pub ch: char,
    /// Darken factor (0.0..1.0).
    pub darken: f32,
}

/// Add a 1-cell shaded edge around visible cells.
pub fn apply_edge_shade(grid: &Grid, shade: EdgeShade) -> Grid {
    let mut out = grid.clone();
    let height = grid.height();
    let width = grid.width();

    for r in 0..height {
        for c in 0..width {
            let Some(cell) = grid.cell(r, c) else {
                continue;
            };
            if !cell.visible {
                continue;
            }
            for (dr, dc) in NEIGHBORS {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr < 0 || nc < 0 {
                    continue;
                }
                let nr = nr as usize;
                let nc = nc as usize;
                let Some(target) = out.cell_mut(nr, nc) else {
                    continue;
                };
                if target.visible {
                    continue;
                }
                target.visible = true;
                target.ch = shade.ch;
                target.fg = cell.fg.map(|color| darken(color, shade.darken));
            }
        }
    }

    out
}

fn darken(color: Color, amount: f32) -> Color {
    let factor = (1.0 - amount.clamp(0.0, 1.0)).clamp(0.0, 1.0);
    match color {
        Color::Rgb(r, g, b) => Color::Rgb(
            (r as f32 * factor).round() as u8,
            (g as f32 * factor).round() as u8,
            (b as f32 * factor).round() as u8,
        ),
        other => other,
    }
}

const NEIGHBORS: &[(i32, i32)] = &[
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
    (-1, -1),
    (-1, 1),
    (1, -1),
    (1, 1),
];
