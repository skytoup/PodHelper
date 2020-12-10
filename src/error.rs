use std::io;

#[derive(Debug)]
pub enum PodError {
    MD5,
    PodfileLock,
    Reqwest(reqwest::Error),
    Io(io::Error),
    Other(&'static str),
}

impl From<reqwest::Error> for PodError {
    fn from(error: reqwest::Error) -> PodError {
        PodError::Reqwest(error)
    }
}

impl From<io::Error> for PodError {
    fn from(error: io::Error) -> PodError {
        PodError::Io(error)
    }
}

pub type Result<T> = std::result::Result<T, PodError>;
