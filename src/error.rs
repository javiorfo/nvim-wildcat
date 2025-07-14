#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Mlua(nvim_oxi::mlua::Error),
    Msg(String),
    BuildTool(String),
}

pub type Result<T = ()> = std::result::Result<T, Error>;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(e) => write!(f, "IO internal error: {e}"),
            Error::Mlua(e) => write!(f, "Mlua internal error: {e}"),
            Error::Msg(e) | Error::BuildTool(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(e) => Some(e),
            Error::Mlua(e) => Some(e),
            Error::Msg(_) => None,
            Error::BuildTool(_) => None,
        }
    }
}
