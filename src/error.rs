use std::error::Error as StdError;
use std::fmt;

use serde::Deserialize;

/// A `Result` alias where the `Err` case is `arnak::Error`.
pub type Result<T> = std::result::Result<T, Error>;

/// An error returned by the API. This is an enum representing
/// either an http error (reqwest::Error), or an error parsing the
/// output (serde_xml_rs::Error), or finally just a string. Which
/// is typically returned in some case an error shouldn't happen.
#[derive(Debug)]
pub enum Error {
    /// An error was returned making the HTTP request, or an error
    /// status code was returned.
    HttpError(reqwest::Error),
    /// An error occured attempting to parse the response from
    /// the API into the expected type.
    UnexpectedResponseError(serde_xml_rs::Error),
    /// The request tried too many times and timed out before the
    /// data was ready to be returned by the API.
    MaxRetryError(u32),
    /// The username requested was not found.
    UnknownUsernameError,
    /// Invalid value supplied for subtype ([crate::GameType]) query parameter.
    InvalidCollectionItemType,
    /// The API returned a list of errors that we do not recognise.
    UnknownApiErrors(Vec<String>),
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::HttpError(err)
    }
}

impl From<serde_xml_rs::Error> for Error {
    fn from(err: serde_xml_rs::Error) -> Self {
        Error::UnexpectedResponseError(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::HttpError(e) => write!(f, "error making request: {}", e),
            Error::UnexpectedResponseError(e) => write!(f, "error parsing output: {}", e),
            Error::MaxRetryError(retries) => {
                write!(f, "data still not ready after {retries} retries, aborting")
            },
            Error::UnknownUsernameError => write!(f, "username not found"),
            Error::InvalidCollectionItemType => write!(f, "invalid collection item type provided"),
            Error::UnknownApiErrors(messages) => match messages.len() {
                0 => write!(f, "got error from API with no message"),
                1 => write!(f, "got unknown error from API: {}", messages[0]),
                _ => write!(f, "got errors from API: {}", messages.join(", ")),
            },
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match &self {
            Error::HttpError(e) => Some(e),
            Error::UnexpectedResponseError(e) => Some(e),
            Error::MaxRetryError(_) => None,
            Error::UnknownUsernameError => None,
            Error::InvalidCollectionItemType => None,
            Error::UnknownApiErrors(_) => None,
        }
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct ApiXmlErrors {
    #[serde(rename = "$value")]
    pub(crate) errors: Vec<ApiXmlError>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ApiXmlError {
    pub(crate) message: String,
}

impl From<ApiXmlErrors> for Error {
    fn from(errors: ApiXmlErrors) -> Self {
        let error_messages: Vec<String> = errors.errors.into_iter().map(|e| e.message).collect();
        if error_messages.len() == 1 {
            let error_message = &error_messages[0];
            if error_message == "Invalid username specified" {
                return Self::UnknownUsernameError;
            }
            if error_message == "Invalid collection subtype" {
                return Self::InvalidCollectionItemType;
            }
        }
        Self::UnknownApiErrors(error_messages)
    }
}
