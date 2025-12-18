#[derive(Debug)]
pub enum RuntimeError {
    Msg(String),
    Io(std::io::Error),
}

impl From<std::io::Error> for RuntimeError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::Msg(s) => write!(f, "{s}"),
            RuntimeError::Io(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for RuntimeError {}

pub type Result<T> = std::result::Result<T, RuntimeError>;
