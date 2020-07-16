use nom::error::{self, ErrorKind};
use serde::{de, ser};
use std::{
    fmt::{self, Display},
    io,
};

pub type Result<T> = std::result::Result<T, Error>;

/// This is a bare-bones implementation. I might come back and improve that later!
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    Parser(ErrorKind),
    Message(String),
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Parser(e) => formatter.write_str(e.description()),
            Error::Message(e) => formatter.write_str(e),
        }
    }
}

impl std::error::Error for Error {}

impl Into<io::Error> for Error {
    fn into(self) -> io::Error {
        io::Error::new(io::ErrorKind::Other, format!("{}", self))
    }
}
