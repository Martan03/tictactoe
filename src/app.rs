use std::{
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
    widgets::{Layout, Paragraph, Spacer, StrSpanExtension},
};

use crate::{
    board::{Board, Cell},
    error::Error,
};

/// Represents game state
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum State {
    Playing,
    Win(Cell),
    Draw,
}

/// App struct containing the main loop, key listeners and rendering
#[derive(Debug)]
pub struct App {
    pub term: Term,
    pub board: Board,
    pub player: Cell,
    pub state: State,
}

impl App {
    /// Creates new [`App`]
    pub fn new() -> Self {
        Self {
            term: Term::new().small_screen(App::small_screen()),
            ..Default::default()
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
        let mut layout = Layout::vertical();
        layout.add_child(Spacer::new(), Constraint::Fill);
        layout.add_child(self.board.clone(), Constraint::Min(0));
        layout.add_child(self.render_state(), Constraint::Min(0));
        layout.add_child(Spacer::new(), Constraint::Fill);

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
            KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('K') => {
                self.board.up();
            }
            KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('J') => {
                self.board.down();
            }
            KeyCode::Right | KeyCode::Char('l') | KeyCode::Char('L') => {
                self.board.right();
            }
            KeyCode::Left | KeyCode::Char('h') | KeyCode::Char('H') => {
                self.board.left();
            }
            KeyCode::Enter => match self.board.set_selected(self.player) {
                Ok(s) => {
                    self.state = s;
                    self.player = self.player.next();
                }
                Err(_) => {}
            },
            KeyCode::Char('r') | KeyCode::Char('R') => {
                self.board.restart();
                self.state = State::Playing;
                self.player = Cell::Cross;
            }
            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
                return Err(Error::Exit)
            }
            _ => return Ok(()),
        }
        self.render()
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            term: Term::new().small_screen(App::small_screen()),
            board: Board::new(3, 3, 3),
            player: Cell::Cross,
            state: State::Playing,
        }
    }
}

impl App {
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
        let (player, msg) = match self.state {
            State::Playing => (self.player, " turn."),
            State::Win(plr) => (plr, " wins!"),
            State::Draw => (Cell::Empty, "Draw!"),
        };

        let player = match player {
            Cell::Circle => "O".fg(Color::Red),
            Cell::Cross => "X".fg(Color::Green),
            _ => "".to_span(),
        };
        Paragraph::new(vec![player.into(), msg.into()]).separator(" ")
    }
}
