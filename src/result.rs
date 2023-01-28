#[derive(Debug)]
pub enum Error {
    Request(reqwest::Error),
    Json(serde_json::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Request(err) => write!(f, "Request error: {err}"),
            Error::Json(err) => write!(f, "Json error: {err}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Request(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Json(err)
    }
}

pub type Result<T = ()> = std::result::Result<T, Error>;
