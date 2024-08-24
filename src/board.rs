use crate::{cell::Cell, error::Error};
use std::cmp::min;
use termint::{geometry::Coords, widgets::Widget};

/// Represents tictactoe board
#[derive(Debug, Clone)]
pub struct Board {
    pub cells: Vec<Cell>,
    pub selected: Coords,
    pub size: Coords,
    pub win_len: usize,
    pub win: Option<(Coords, (isize, isize))>,
    state: Option<Cell>,
}

impl Board {
    /// Creates new [`Board`]
    pub fn new(width: usize, height: usize, win_len: usize) -> Self {
        Self {
            cells: vec![Cell::Empty; width * height],
            selected: Coords::new(width / 2, height / 2),
            size: Coords::new(width, height),
            win_len,
            win: None,
            state: None,
        }
    }

    /// Restarts the game
    pub fn restart(&mut self) {
        self.cells = vec![Cell::Empty; self.size.x * self.size.y];
        self.state = None;
        self.win = None;
    }

    /// Sets cell on given coordinates to given value
    pub fn set(
        &mut self,
        cell: Cell,
        x: usize,
        y: usize,
    ) -> Result<Option<Cell>, Error> {
        if self.state.is_some() {
            return Err(Error::Msg("game ended".into()));
        }

        let id = x + y * self.size.x;
        match self.cells[id] {
            Cell::Empty => {
                self.cells[id] = cell;
                self.state = self.check_state();
                Ok(self.state)
            }
            _ => Err(Error::Msg(String::from("Not empty cell"))),
        }
    }

    /// Sets selected cell to given value
    pub fn set_selected(&mut self, cell: Cell) -> Result<Option<Cell>, Error> {
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
        self.selected.y = min(self.selected.y + 1, self.size.y - 1);
    }

    /// Moves selected up
    pub fn left(&mut self) {
        self.selected.x = self.selected.x.saturating_sub(1);
    }

    /// Moves selected up
    pub fn right(&mut self) {
        self.selected.x = min(self.selected.x + 1, self.size.x - 1);
    }

    /// Gets game state
    pub fn state(&self) -> Option<Cell> {
        self.state
    }
}

impl Board {
    /// Checks game state
    fn check_state(&mut self) -> Option<Cell> {
        let mut draw = true;
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                if self.cells[x + y * self.size.x] == Cell::Empty {
                    draw = false;
                    continue;
                }

                if x + self.win_len <= self.size.x
                    && self.check_win(x, y, 1, 0)
                {
                    return Some(self.cells[x + y * self.size.x]);
                }

                if y + self.win_len <= self.size.y {
                    if self.check_win(x, y, 0, 1) {
                        return Some(self.cells[x + y * self.size.x]);
                    }

                    if x + self.win_len <= self.size.x
                        && self.check_win(x, y, 1, 1)
                    {
                        return Some(self.cells[x + y * self.size.x]);
                    }

                    if x + 1 >= self.win_len && self.check_win(x, y, -1, 1) {
                        return Some(self.cells[x + y * self.size.x]);
                    }
                }
            }
        }

        (draw).then_some(Cell::Empty)
    }

    /// Checks win from given position and with given direction
    fn check_win(
        &mut self,
        mut x: usize,
        mut y: usize,
        xd: isize,
        yd: isize,
    ) -> bool {
        let pos = Coords::new(x, y);
        let cell = self.cells[x + y * self.size.x];
        x = (x as isize + xd) as usize;
        y = (y as isize + yd) as usize;
        for _ in 1..self.win_len {
            if self.cells[x + y * self.size.x] != cell {
                return false;
            }
            x = (x as isize + xd) as usize;
            y = (y as isize + yd) as usize;
        }

        self.win = Some((pos, (xd, yd)));
        true
    }
}

impl From<Board> for Box<dyn Widget> {
    fn from(value: Board) -> Self {
        Box::new(value)
    }
}
