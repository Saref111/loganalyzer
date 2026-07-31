use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum AppError {
    IO(std::io::Error),
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IO(err) => write!(f, "{}", err),
        }
    }
}

impl Error for AppError {}

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        AppError::IO(value)
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
