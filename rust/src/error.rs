#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Msg(String),
}

pub type Result<T = ()> = std::result::Result<T, Error>;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(e) => write!(f, "IO internal error: {}", e),
            Error::Msg(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(e) => Some(e),
            Error::Msg(_) => None,
        }
    }
}
