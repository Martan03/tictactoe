use termint::{
    enums::Color,
    help,
    widgets::{Grad, StrSpanExtension},
};

use crate::error::Error;

/// Parses given arguments and checks for arguments conditions
#[derive(Debug)]
pub struct Args {
    pub width: Option<usize>,
    pub height: Option<usize>,
    pub win_len: usize,
    pub help: bool,
}

impl Args {
    /// Parses arguments
    pub fn parse(args: std::env::Args) -> Result<Args, Error> {
        let mut parsed = Self::default();

        let mut args_iter = args.into_iter();
        args_iter.next();
        while let Some(arg) = args_iter.next() {
            match arg.as_str() {
                "-s" | "--size" => parsed.parse_size(&mut args_iter)?,
                "-w" | "--win" => parsed.parse_win(&mut args_iter)?,
                "-h" | "--help" => parsed.help = true,
                arg => Err(format!("unexpected argument: '{arg}'"))?,
            }
        }

        Ok(parsed)
    }

    /// Displays help
    pub fn help() {
        println!(
            "Welcome to help for {} by {}\n",
            "tictactoe".fg(Color::Green),
            Grad::new("Martan03", (0, 220, 255), (175, 80, 255))
        );
        help!(
            "Usage":
            "tictactoe" => "Opens 3x3 game with win length set to 3\n"
            "tictactoe" ["options"] => "Behaves according to options\n"
            "Options":
            "-s  --size" => "Sets size of the game\n"
            "-w  --win" => "Sets win length\n"
            "-h  --help" => "Prints this help"
        );
    }

    /// Parses size from the given arguments
    fn parse_size<T>(&mut self, args: &mut T) -> Result<(), Error>
    where
        T: Iterator<Item = String>,
    {
        self.width = Some(Args::get_num(args)?);
        self.height = Some(Args::get_num(args)?);

        if self.width.unwrap() < 3 || self.height.unwrap() < 3 {
            return Err(Error::Msg("minimum supported size is 3".into()));
        }
        Ok(())
    }

    /// Parses win length from the given arguments
    fn parse_win<T>(&mut self, args: &mut T) -> Result<(), Error>
    where
        T: Iterator<Item = String>,
    {
        self.win_len = Args::get_num(args)?;
        if self.win_len < 3 {
            return Err(Error::Msg(
                "minimum supported win length is 3".into(),
            ));
        }
        Ok(())
    }

    /// Gets number (usize) from args
    fn get_num<T>(args: &mut T) -> Result<usize, Error>
    where
        T: Iterator<Item = String>,
    {
        let Some(val) = args.next() else {
            return Err(Error::Msg("missing argument parameter".into()));
        };

        val.parse::<usize>()
            .map_err(|_| Error::Msg(format!("number expected, got '{val}'")))
    }
}

impl Default for Args {
    fn default() -> Self {
        Self {
            width: None,
            height: None,
            win_len: 5,
            help: false,
        }
    }
}
