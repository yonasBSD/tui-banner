use crate::color::Color;

#[derive(Clone, Debug)]
pub struct Cell {
    pub ch: char,
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    pub visible: bool,
}

#[derive(Clone, Debug)]
pub struct Grid {
    cells: Vec<Vec<Cell>>,
}

#[derive(Clone, Copy, Debug)]
pub enum Align {
    Left,
    Center,
    Right,
}

#[derive(Clone, Copy, Debug)]
pub struct Padding {
    pub top: usize,
    pub bottom: usize,
    pub left: usize,
    pub right: usize,
}

impl Grid {
    pub fn new(height: usize, width: usize) -> Self {
        let row = vec![
            Cell {
                ch: ' ',
                fg: None,
                bg: None,
                visible: false,
            };
            width
        ];
        let cells = vec![row; height];
        Self { cells }
    }

    pub fn from_char_rows(rows: Vec<Vec<char>>) -> Self {
        let cells = rows
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|ch| Cell {
                        ch,
                        fg: None,
                        bg: None,
                        visible: ch != ' ',
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Self { cells }
    }

    pub fn height(&self) -> usize {
        self.cells.len()
    }

    pub fn width(&self) -> usize {
        self.cells.first().map(|row| row.len()).unwrap_or(0)
    }

    pub fn cell_mut(&mut self, row: usize, col: usize) -> Option<&mut Cell> {
        self.cells.get_mut(row).and_then(|r| r.get_mut(col))
    }

    pub fn cell(&self, row: usize, col: usize) -> Option<&Cell> {
        self.cells.get(row).and_then(|r| r.get(col))
    }

    pub fn rows(&self) -> &[Vec<Cell>] {
        &self.cells
    }

    pub fn rows_mut(&mut self) -> &mut [Vec<Cell>] {
        &mut self.cells
    }

    pub fn blit(&mut self, other: &Grid, top: usize, left: usize) {
        for (r, row) in other.cells.iter().enumerate() {
            let target_r = top + r;
            if target_r >= self.height() {
                continue;
            }
            for (c, cell) in row.iter().enumerate() {
                let target_c = left + c;
                if target_c >= self.width() {
                    continue;
                }
                if cell.visible {
                    self.cells[target_r][target_c] = cell.clone();
                }
            }
        }
    }
}

impl Padding {
    pub fn uniform(value: usize) -> Self {
        Self {
            top: value,
            bottom: value,
            left: value,
            right: value,
        }
    }
}

impl From<usize> for Padding {
    fn from(value: usize) -> Self {
        Padding::uniform(value)
    }
}

impl From<(usize, usize, usize, usize)> for Padding {
    fn from(values: (usize, usize, usize, usize)) -> Self {
        Self {
            top: values.0,
            right: values.1,
            bottom: values.2,
            left: values.3,
        }
    }
}
