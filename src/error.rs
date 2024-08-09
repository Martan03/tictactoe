use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Msg(String),
    Exit,
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value)
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::Msg(value.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IO(e) => write!(f, "{e}"),
            Error::Msg(msg) => write!(f, "{msg}"),
            Error::Exit => write!(f, "exit"),
        }
    }
}
