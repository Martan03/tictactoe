use std::{
    cmp::{max, min},
    io::{stdout, Write},
    time::Duration,
};

use crossterm::{
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use termint::{
    enums::{Color, Modifier},
    geometry::{Constraint, Coords, TextAlign},
    term::Term,
    widgets::{Layout, Paragraph, StrSpanExtension, Widget},
};

use crate::{board::Board, cell::Cell, error::Error};

/// App struct containing the main loop, key listeners and rendering
#[derive(Debug)]
pub struct App {
    pub term: Term,
    pub board: Board,
    pub player: Cell,
}

impl App {
    /// Creates new [`App`] with board with given size and win length
    pub fn new(size: Option<Coords>, win: Option<usize>) -> Self {
        let (w, h) = match size {
            Some(c) => (c.x, c.y),
            _ => App::fullscreen_size(),
        };
        let win = win.unwrap_or(min(max(w, h), 5));

        Self {
            term: Term::new().small_screen(App::small_screen()),
            board: Board::new(w, h, win),
            player: Cell::Cross,
        }
    }

    /// Runs the [`App`]
    pub fn run(&mut self) -> Result<(), Error> {
        // Saves screen, clears screen and hides cursor
        print!("\x1b[?1049h\x1b[2J\x1b[?25l");
        _ = stdout().flush();
        enable_raw_mode()?;

        let res = self.main_loop();

        disable_raw_mode()?;
        // Restores screen
        print!("\x1b[?1049l\x1b[?25h");
        _ = stdout().flush();

        match res {
            Err(Error::Exit) => Ok(()),
            _ => res,
        }
    }

    /// Main loop of the [`App`]
    fn main_loop(&mut self) -> Result<(), Error> {
        self.render()?;
        loop {
            if poll(Duration::from_millis(100))? {
                self.event()?;
            }
        }
    }

    /// Renders current screen of the [`App`]
    pub fn render(&mut self) -> Result<(), Error> {
        let mut layout = Layout::vertical().center();
        layout.add_child(self.render_state(), Constraint::Min(0));
        layout.add_child(self.board.clone(), Constraint::Min(0));

        let mut center = Layout::horizontal().center();
        center.add_child(layout, Constraint::Min(0));

        let mut main = Layout::vertical();
        main.add_child(center, Constraint::Fill);
        main.add_child(Self::render_help(), Constraint::Min(0));

        self.term.render(main)?;
        Ok(())
    }

    /// Handles key listening
    fn event(&mut self) -> Result<(), Error> {
        match read()? {
            Event::Key(e) => self.key_handler(e),
            Event::Resize(_, _) => self.render(),
            _ => Ok(()),
        }
    }
}

impl App {
    /// Handles key events
    fn key_handler(&mut self, event: KeyEvent) -> Result<(), Error> {
        match event.code {
            KeyCode::Up | KeyCode::Char('k') => self.board.up(),
            KeyCode::Down | KeyCode::Char('j') => self.board.down(),
            KeyCode::Right | KeyCode::Char('l') => self.board.right(),
            KeyCode::Left | KeyCode::Char('h') => self.board.left(),
            KeyCode::Enter => {
                if self.board.set_selected(self.player).is_ok() {
                    self.player = self.player.next();
                }
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                self.board.restart();
                self.player = Cell::Cross;
            }
            KeyCode::Char('c')
                if event.modifiers.contains(KeyModifiers::CONTROL) =>
            {
                return Err(Error::Exit);
            }
            KeyCode::Esc | KeyCode::Char('q') => return Err(Error::Exit),
            _ => return Ok(()),
        }
        self.render()
    }

    /// Gets board size based on the current screen size.
    /// Minimum size is based on the win size.
    fn fullscreen_size() -> (usize, usize) {
        Term::get_size()
            .map(|(w, h)| {
                (
                    max(w.saturating_sub(1) / 4, 3),
                    max(
                        h.saturating_sub(
                            2 + Self::render_help().height(&Coords::new(w, h)),
                        ) / 2,
                        3,
                    ),
                )
            })
            .unwrap_or((3, 3))
    }

    /// Small screen to be displayed, when game can't fit
    fn small_screen() -> Layout {
        let mut layout = Layout::vertical().center();
        layout.add_child(
            "Terminal too small!"
                .modifier(Modifier::BOLD)
                .align(TextAlign::Center),
            Constraint::Min(0),
        );
        layout.add_child(
            "You have to increase terminal size".align(TextAlign::Center),
            Constraint::Min(0),
        );
        layout
    }

    /// Renders game state text
    fn render_state(&self) -> Paragraph {
        let (player, msg) = match self.board.state() {
            Some(Cell::Empty) => (Cell::Empty, "Draw!"),
            None => (self.player, " turn."),
            Some(plr) => (plr, " wins!"),
        };

        let player = match player {
            Cell::Circle => "O".fg(Color::Red),
            Cell::Cross => "X".fg(Color::Green),
            _ => "".to_span(),
        };
        Paragraph::new(vec![player.into(), msg.into()]).separator(" ")
    }

    /// Renders help with all the keybinds
    fn render_help() -> Paragraph {
        Paragraph::new(vec![
            "[Arrows/hjkl]Move".fg(Color::Gray).into(),
            "[Enter]Place".fg(Color::Gray).into(),
            "[r]Restart".fg(Color::Gray).into(),
            "[Esc|q]Quit".fg(Color::Gray).into(),
        ])
        .separator("  ")
    }
}
