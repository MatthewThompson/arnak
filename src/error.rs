use std::{error::Error, fmt};

/// Result containing BoardGameGeekApiError on failure.
pub type Result<T> = std::result::Result<T, BoardGameGeekApiError>;

/// An error returned by the API. This is an enum representing
/// either an http error (reqwest::Error), or an error parsing the
/// output (serde_xml_rs::Error), or finally just a string. Which
/// is typically returned in some case an error shouldn't happen.
#[derive(Debug)]
pub enum BoardGameGeekApiError {
    HttpError(reqwest::Error),
    ParseError(serde_xml_rs::Error),
    ApiError(String),
}

impl From<reqwest::Error> for BoardGameGeekApiError {
    fn from(err: reqwest::Error) -> Self {
        BoardGameGeekApiError::HttpError(err)
    }
}

impl From<serde_xml_rs::Error> for BoardGameGeekApiError {
    fn from(err: serde_xml_rs::Error) -> Self {
        BoardGameGeekApiError::ParseError(err)
    }
}

impl fmt::Display for BoardGameGeekApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BoardGameGeekApiError::HttpError(e) => write!(f, "error making request: {}", e),
            BoardGameGeekApiError::ParseError(e) => write!(f, "error parsing output: {}", e),
            BoardGameGeekApiError::ApiError(s) => write!(f, "{s}"),
        }
    }
}

impl Error for BoardGameGeekApiError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self {
            BoardGameGeekApiError::HttpError(e) => Some(e),
            BoardGameGeekApiError::ParseError(e) => Some(e),
            BoardGameGeekApiError::ApiError(_) => None,
        }
    }
}
