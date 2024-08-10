use crate::{board::Board, cell::Cell};
use termint::{
    buffer::Buffer, enums::Color, geometry::Coords, style::Style,
    widgets::Widget,
};

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
    /// Renders the line over the winning sequence
    fn render_win(&self, buffer: &mut Buffer) {
        let Some(pd) = self.win else {
            return;
        };

        match pd.1 {
            (1, 0) => self.cross_hor(buffer, &pd.0),
            (0, 1) => self.cross_win(buffer, pd, '|', ' ', ' ', (2, 0)),
            (1, 1) => self.cross_win(buffer, pd, '\\', '`', '⹁', (0, 0)),
            (-1, 1) => self.cross_win(buffer, pd, '/', ',', '\'', (4, 0)),
            _ => {}
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

    /// Renders horizontal win
    fn cross_hor(&self, buffer: &mut Buffer, pos: &Coords) {
        let color = self.win_color(pos);
        let mut pos = Coords::new(
            buffer.x() + pos.x * 4 + 1,
            buffer.y() + pos.y * 2 + 1,
        );
        for _ in 0..self.win_len * 2 {
            Self::render_cell(buffer, '-', color, &pos);
            pos.x += 2;
        }
    }

    /// Crosses the winning sequence
    fn cross_win(
        &self,
        buffer: &mut Buffer,
        (pos, (dx, dy)): (Coords, (isize, isize)),
        bc: char,
        fc: char,
        ac: char,
        (ox, oy): (usize, usize),
    ) {
        let color = self.win_color(&pos);
        let mut p = Coords::new(
            buffer.x() + pos.x * 4 + ox,
            buffer.y() + pos.y * 2 + oy,
        );

        for _ in 0..self.win_len {
            Self::render_cell(buffer, bc, color, &p);
            p.x = (p.x as isize + dx * 2) as usize;
            p.y = (p.y as isize + dy) as usize;

            Self::render_cell(buffer, fc, color, &Coords::new(p.x - 1, p.y));
            Self::render_cell(buffer, ac, color, &Coords::new(p.x + 1, p.y));
            p.x = (p.x as isize + dx * 2) as usize;
            p.y = (p.y as isize + dy) as usize;
        }
        Self::render_cell(buffer, bc, color, &p);
    }

    /// Gets win line color
    fn win_color(&self, pos: &Coords) -> Color {
        match self.cells[pos.x + pos.y * self.size.x] {
            Cell::Cross => Color::Green,
            Cell::Circle => Color::Red,
            Cell::Empty => Color::Default,
        }
    }

    /// Renders value and color to cell on given position
    fn render_cell(buffer: &mut Buffer, val: char, col: Color, pos: &Coords) {
        buffer.set_val(val, pos);
        buffer.set_fg(col, pos);
    }
}
