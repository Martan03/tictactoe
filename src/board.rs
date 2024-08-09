use std::cmp::min;

use termint::{
    buffer::Buffer, enums::Color, geometry::Coords, style::Style,
    widgets::Widget,
};

use crate::{app::State, error::Error};

/// Represents cell value
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Cell {
    Cross,
    Circle,
    Empty,
}

impl Cell {
    /// Gets next cell value on play
    pub fn next(self) -> Self {
        match self {
            Cell::Cross => Cell::Circle,
            _ => Cell::Cross,
        }
    }
}

/// Represents tictactoe board
#[derive(Debug, Clone)]
pub struct Board {
    cells: Vec<Cell>,
    selected: Coords,
    width: usize,
    height: usize,
    win_len: usize,
}

impl Board {
    /// Creates new [`Board`]
    pub fn new(width: usize, height: usize, win_len: usize) -> Self {
        Self {
            cells: vec![Cell::Empty; width * height],
            selected: Coords::new(0, 0),
            width,
            height,
            win_len,
        }
    }

    /// Restarts the game
    pub fn restart(&mut self) {
        self.cells = vec![Cell::Empty; self.width * self.height];
    }

    /// Sets cell on given coordinates to given value
    pub fn set(
        &mut self,
        cell: Cell,
        x: usize,
        y: usize,
    ) -> Result<State, Error> {
        let id = x + y * self.width;
        match self.cells[id] {
            Cell::Empty => {
                self.cells[id] = cell;
                Ok(self.check_state())
            }
            _ => Err(Error::Msg(String::from("Not empty cell"))),
        }
    }

    /// Sets selected cell to given value
    pub fn set_selected(&mut self, cell: Cell) -> Result<State, Error> {
        self.set(cell, self.selected.x, self.selected.y)
    }

    /// Sets selected cell
    pub fn select(&mut self, coords: Coords) {
        self.selected = coords;
    }

    /// Moves selected up
    pub fn up(&mut self) {
        self.selected.y = self.selected.y.saturating_sub(1);
    }

    /// Moves selected up
    pub fn down(&mut self) {
        self.selected.y = min(self.selected.y + 1, self.height - 1);
    }

    /// Moves selected up
    pub fn left(&mut self) {
        self.selected.x = self.selected.x.saturating_sub(1);
    }

    /// Moves selected up
    pub fn right(&mut self) {
        self.selected.x = min(self.selected.x + 1, self.width - 1);
    }
}

impl Widget for Board {
    fn render(&self, buffer: &mut Buffer) {
        self.render_inner(buffer);
        self.render_outer(buffer);
        self.render_cells(buffer);
        self.render_sel(buffer);
    }

    fn height(&self, _size: &Coords) -> usize {
        self.height * 2 + 1
    }

    fn width(&self, _size: &Coords) -> usize {
        self.width * 4 + 1
    }
}

impl Board {
    /// Checks game state
    fn check_state(&self) -> State {
        let mut draw = true;
        for y in 0..self.height {
            for x in 0..self.width {
                if self.cells[x + y * self.width] == Cell::Empty {
                    draw = false;
                    continue;
                }

                if x + self.win_len <= self.width && self.check_win(x, y, 1, 0)
                {
                    return State::Win(self.cells[x + y * self.width]);
                }

                if y + self.win_len <= self.height {
                    if self.check_win(x, y, 0, 1) {
                        return State::Win(self.cells[x + y * self.width]);
                    }

                    if x + self.win_len <= self.width
                        && self.check_win(x, y, 1, 1)
                    {
                        return State::Win(self.cells[x + y * self.width]);
                    }

                    if x + 1 >= self.win_len && self.check_win(x, y, -1, 1) {
                        return State::Win(self.cells[x + y * self.width]);
                    }
                }
            }
        }

        match draw {
            true => State::Draw,
            false => State::Playing,
        }
    }

    /// Checks win from given position and with given direction
    fn check_win(
        &self,
        mut x: usize,
        mut y: usize,
        xd: isize,
        yd: isize,
    ) -> bool {
        let cell = self.cells[x + y * self.width];
        x = (x as isize + xd) as usize;
        y = (y as isize + yd) as usize;
        for _ in 1..self.win_len {
            if self.cells[x + y * self.width] != cell {
                return false;
            }
            x = (x as isize + xd) as usize;
            y = (y as isize + yd) as usize;
        }
        true
    }

    /// Renders selected border
    fn render_sel(&self, buffer: &mut Buffer) {
        let sel_x = buffer.x() + self.selected.x * 4;
        let sel_y = buffer.y() + self.selected.y * 2;

        let (top, bottom) = match (self.selected.x, self.selected.y) {
            (0, 0) => ("┏━━━┱", "┡━━━╃"),
            (0, y) if y + 1 == self.height => ("┢━━━╅", "┗━━━┹"),
            (x, 0) if x + 1 == self.width => ("┲━━━┓", "╄━━━┩"),
            (x, y) if x + 1 == self.width && y + 1 == self.height => {
                ("╆━━━┪", "┺━━━┛")
            }
            (_, 0) => ("┲━━━┱", "╄━━━╃"),
            (_, y) if y + 1 == self.height => ("╆━━━╅", "┺━━━┹"),
            (0, _) => ("┢━━━╅", "┡━━━╃"),
            (x, _) if x + 1 == self.width => ("╆━━━┪", "╄━━━┩"),
            _ => ("╆━━━╅", "╄━━━╃"),
        };
        buffer.set_str(top, &Coords::new(sel_x, sel_y));
        buffer.set_str(bottom, &Coords::new(sel_x, sel_y + 2));
        buffer.set_val('┃', &Coords::new(sel_x, sel_y + 1));
        buffer.set_val('┃', &Coords::new(sel_x + 4, sel_y + 1));
    }

    /// Renders cells
    fn render_cells(&self, buffer: &mut Buffer) {
        let mut coords = Coords::new(buffer.x() + 2, buffer.y() + 1);
        let mut id = 0;
        for _ in 0..self.height {
            for _ in 0..self.width {
                match self.cells[id] {
                    Cell::Cross => buffer.set_str_styled(
                        "X",
                        &coords,
                        Style::new().fg(Color::Green),
                    ),
                    Cell::Circle => buffer.set_str_styled(
                        "O",
                        &coords,
                        Style::new().fg(Color::Red),
                    ),
                    Cell::Empty => {}
                }
                id += 1;
                coords.x += 4;
            }
            coords.y += 2;
            coords.x = buffer.x() + 2;
        }
    }

    /// Renders outer borders
    fn render_outer(&self, buffer: &mut Buffer) {
        let bottom = self.height * 2;
        let right = self.width * 4;

        buffer.set_str_styled(
            "───┬".repeat(self.width),
            &Coords::new(buffer.x() + 1, buffer.y()),
            Style::new().fg(Color::Gray),
        );
        buffer.set_str_styled(
            "───┴".repeat(self.width),
            &Coords::new(buffer.x() + 1, buffer.y() + bottom),
            Style::new().fg(Color::Gray),
        );

        let mut leftc = Coords::new(buffer.x(), buffer.y() + 1);
        let mut rightc = Coords::new(buffer.x() + right, buffer.y() + 1);
        for _ in buffer.y()..buffer.y() + self.height {
            Board::border_part('│', buffer, &leftc);
            leftc.y += 1;
            Board::border_part('├', buffer, &leftc);
            leftc.y += 1;

            Board::border_part('│', buffer, &rightc);
            rightc.y += 1;
            Board::border_part('┤', buffer, &rightc);
            rightc.y += 1;
        }

        let mut pos = buffer.pos();
        Board::border_part('┌', buffer, &pos);
        pos.x += right;
        Board::border_part('┐', buffer, &pos);
        pos.y += bottom;
        Board::border_part('┘', buffer, &pos);
        pos.x -= right;
        Board::border_part('└', buffer, &pos);
    }

    /// Renders inner borders
    fn render_inner(&self, buffer: &mut Buffer) {
        let line = "───┼".repeat(self.width);
        for y in 1..self.height {
            buffer.set_str_styled(
                &line,
                &Coords::new(buffer.x() + 1, buffer.y() + y * 2),
                Style::new().fg(Color::Gray),
            );
        }

        let line = "   │".repeat(self.width);
        for y in 0..self.height {
            buffer.set_str_styled(
                &line,
                &Coords::new(buffer.x() + 1, buffer.y() + y * 2 + 1),
                Style::new().fg(Color::Gray),
            )
        }
    }

    /// Renders part of the border
    fn border_part(val: char, buffer: &mut Buffer, pos: &Coords) {
        buffer.set_val(val, pos);
        buffer.set_fg(Color::Gray, pos);
    }
}

impl From<Board> for Box<dyn Widget> {
    fn from(value: Board) -> Self {
        Box::new(value)
    }
}
