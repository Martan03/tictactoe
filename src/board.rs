use std::cmp::min;

use termint::{
    buffer::Buffer, enums::Color, geometry::Coords, style::Style,
    widgets::Widget,
};

use crate::error::Error;

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
    size: Coords,
    win_len: usize,
    win: Option<(Coords, (isize, isize))>,
    state: Option<Cell>,
}

impl Board {
    /// Creates new [`Board`]
    pub fn new(width: usize, height: usize, win_len: usize) -> Self {
        Self {
            cells: vec![Cell::Empty; width * height],
            selected: Coords::new(0, 0),
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
    ) -> Result<(), Error> {
        if self.state.is_some() {
            return Err(Error::Msg("game ended".into()));
        }

        let id = x + y * self.size.x;
        match self.cells[id] {
            Cell::Empty => {
                self.cells[id] = cell;
                self.state = self.check_state();
                Ok(())
            }
            _ => Err(Error::Msg(String::from("Not empty cell"))),
        }
    }

    /// Sets selected cell to given value
    pub fn set_selected(&mut self, cell: Cell) -> Result<(), Error> {
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

impl Widget for Board {
    fn render(&self, buffer: &mut Buffer) {
        self.render_inner(buffer);
        self.render_outer(buffer);
        self.render_cells(buffer);
        self.render_sel(buffer);
        self.render_win(buffer);
    }

    fn height(&self, _size: &Coords) -> usize {
        self.size.y * 2 + 1
    }

    fn width(&self, _size: &Coords) -> usize {
        self.size.x * 4
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

    fn render_win(&self, buffer: &mut Buffer) {
        let Some((pos, dir)) = self.win else {
            return;
        };

        match dir {
            (1, 0) => self.render_win_hor(buffer, &pos),
            (0, 1) => self.render_win_ver(buffer, &pos),
            (1, 1) => self.render_win_db(buffer, &pos),
            _ => self.render_win_df(buffer, &pos),
        }
    }

    /// Renders selected border
    fn render_sel(&self, buffer: &mut Buffer) {
        let sel_x = buffer.x() + self.selected.x * 4;
        let sel_y = buffer.y() + self.selected.y * 2;

        let (top, bottom) = match (self.selected.x, self.selected.y) {
            (0, 0) => ("┏━━━┱", "┡━━━╃"),
            (0, y) if y + 1 == self.size.y => ("┢━━━╅", "┗━━━┹"),
            (x, 0) if x + 1 == self.size.x => ("┲━━━┓", "╄━━━┩"),
            (x, y) if x + 1 == self.size.x && y + 1 == self.size.y => {
                ("╆━━━┪", "┺━━━┛")
            }
            (_, 0) => ("┲━━━┱", "╄━━━╃"),
            (_, y) if y + 1 == self.size.y => ("╆━━━╅", "┺━━━┹"),
            (0, _) => ("┢━━━╅", "┡━━━╃"),
            (x, _) if x + 1 == self.size.x => ("╆━━━┪", "╄━━━┩"),
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
        for _ in 0..self.size.y {
            for _ in 0..self.size.x {
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
        let bottom = self.size.y * 2;
        let right = self.size.x * 4;

        buffer.set_str_styled(
            "───┬".repeat(self.size.x),
            &Coords::new(buffer.x() + 1, buffer.y()),
            Style::new().fg(Color::Gray),
        );
        buffer.set_str_styled(
            "───┴".repeat(self.size.x),
            &Coords::new(buffer.x() + 1, buffer.y() + bottom),
            Style::new().fg(Color::Gray),
        );

        let mut leftc = Coords::new(buffer.x(), buffer.y() + 1);
        let mut rightc = Coords::new(buffer.x() + right, buffer.y() + 1);
        for _ in buffer.y()..buffer.y() + self.size.y {
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
        let line = "───┼".repeat(self.size.x);
        for y in 1..self.size.y {
            buffer.set_str_styled(
                &line,
                &Coords::new(buffer.x() + 1, buffer.y() + y * 2),
                Style::new().fg(Color::Gray),
            );
        }

        let line = "   │".repeat(self.size.x);
        for y in 0..self.size.y {
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

    /// Gets win line color
    fn win_color(&self, pos: &Coords) -> Color {
        match self.cells[pos.x + pos.y * self.size.x] {
            Cell::Cross => Color::Green,
            Cell::Circle => Color::Red,
            Cell::Empty => Color::Default,
        }
    }

    /// Renders horizontal win
    fn render_win_hor(&self, buffer: &mut Buffer, pos: &Coords) {
        let color = self.win_color(pos);
        let mut pos = Coords::new(
            buffer.x() + pos.x * 4 + 1,
            buffer.y() + pos.y * 2 + 1,
        );
        for _ in 0..self.win_len * 2 {
            buffer.set_val('-', &pos);
            buffer.set_fg(color, &pos);
            pos.x += 2;
        }
    }

    /// Renders vertical win
    fn render_win_ver(&self, buffer: &mut Buffer, pos: &Coords) {
        let color = self.win_color(pos);
        let mut pos =
            Coords::new(buffer.x() + pos.x * 4 + 2, buffer.y() + pos.y * 2);
        for _ in 0..self.win_len + 1 {
            buffer.set_val('|', &pos);
            buffer.set_fg(color, &pos);
            pos.y += 2;
        }
    }

    /// Renders back diagonal (backslash) win
    fn render_win_db(&self, buffer: &mut Buffer, pos: &Coords) {
        let color = self.win_color(pos);
        let mut coords =
            Coords::new(buffer.x() + pos.x * 4, buffer.y() + pos.y * 2);
        for _ in 0..self.win_len + 1 {
            buffer.set_val('\\', &coords);
            buffer.set_fg(color, &coords);
            coords.x += 4;
            coords.y += 2;
        }

        let mut coords = Coords::new(
            buffer.x() + pos.x * 4 + 1,
            buffer.y() + pos.y * 2 + 1,
        );
        for _ in 0..self.win_len {
            buffer.set_val('`', &coords);
            buffer.set_fg(color, &coords);
            coords.x += 2;
            buffer.set_val('⹁', &coords);
            buffer.set_fg(color, &coords);
            coords.x += 2;
            coords.y += 2;
        }
    }

    /// Renders back diagonal (backslash) win
    fn render_win_df(&self, buffer: &mut Buffer, pos: &Coords) {
        let color = self.win_color(pos);
        let mut coords =
            Coords::new(buffer.x() + pos.x * 4 + 4, buffer.y() + pos.y * 2);
        for _ in 0..self.win_len + 1 {
            buffer.set_val('/', &coords);
            buffer.set_fg(color, &coords);
            coords.x -= 4;
            coords.y += 2;
        }

        let mut coords = Coords::new(
            buffer.x() + pos.x * 4 + 3,
            buffer.y() + pos.y * 2 + 1,
        );
        for _ in 0..self.win_len {
            buffer.set_val('\'', &coords);
            buffer.set_fg(color, &coords);
            coords.x -= 2;
            buffer.set_val(',', &coords);
            buffer.set_fg(color, &coords);
            coords.x -= 2;
            coords.y += 2;
        }
    }
}

impl From<Board> for Box<dyn Widget> {
    fn from(value: Board) -> Self {
        Box::new(value)
    }
}
