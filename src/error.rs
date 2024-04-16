use std::error::Error as StdError;
use std::fmt;

/// A `Result` alias where the `Err` case is `arnak::Error`.
pub type Result<T> = std::result::Result<T, Error>;

/// An error returned by the API. This is an enum representing
/// either an http error (reqwest::Error), or an error parsing the
/// output (serde_xml_rs::Error), or finally just a string. Which
/// is typically returned in some case an error shouldn't happen.
#[derive(Debug)]
pub enum Error {
    HttpError(reqwest::Error),
    ParseError(serde_xml_rs::Error),
    ApiError(String),
    LocalError(String),
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::HttpError(err)
    }
}

impl From<serde_xml_rs::Error> for Error {
    fn from(err: serde_xml_rs::Error) -> Self {
        Error::ParseError(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::HttpError(e) => write!(f, "error making request: {}", e),
            Error::ParseError(e) => write!(f, "error parsing output: {}", e),
            Error::ApiError(s) => write!(f, "{s}"),
            Error::LocalError(s) => write!(f, "{s}"),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match &self {
            Error::HttpError(e) => Some(e),
            Error::ParseError(e) => Some(e),
            Error::ApiError(_) => None,
            Error::LocalError(_) => None,
        }
    }
}
