use std::{
    cmp::max,
    io::{stdout, Write},
    time::Duration,
};

use crossterm::{
    event::{poll, read, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use termint::{
    enums::{Color, Modifier},
    geometry::{Constraint, TextAlign},
    term::Term,
    widgets::{Layout, Paragraph, StrSpanExtension},
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
    pub fn new(
        width: Option<usize>,
        height: Option<usize>,
        win: usize,
    ) -> Self {
        let (w, h) = match (width, height) {
            (Some(w), Some(h)) => (w, h),
            _ => App::fullscreen_size(win),
        };

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
                self.key_listener()?;
            }
        }
    }

    /// Renders current screen of the [`App`]
    pub fn render(&mut self) -> Result<(), Error> {
        let mut layout = Layout::vertical().center();
        layout.add_child(self.board.clone(), Constraint::Min(0));
        layout.add_child(self.render_state(), Constraint::Min(0));

        let mut main = Layout::horizontal().center();
        main.add_child(layout, Constraint::Min(0));

        self.term.render(main)?;
        Ok(())
    }

    /// Handles key listening
    fn key_listener(&mut self) -> Result<(), Error> {
        let Event::Key(KeyEvent { code, .. }) = read()? else {
            return Ok(());
        };

        match code {
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
            KeyCode::Esc | KeyCode::Char('q') => return Err(Error::Exit),
            _ => return Ok(()),
        }
        self.render()
    }
}

impl App {
    /// Gets board size based on the current screen size.
    /// Minimum size is based on the win size.
    fn fullscreen_size(win: usize) -> (usize, usize) {
        Term::get_size()
            .map(|(w, h)| {
                (
                    max(w.saturating_sub(1) / 4, win),
                    max(h.saturating_sub(2) / 2, win),
                )
            })
            .unwrap_or((win, win))
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
}
