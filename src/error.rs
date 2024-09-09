use std::error::Error as StdError;
use std::fmt;

use serde::Deserialize;
use serde_xml_rs::from_str;

/// A [`std::result::Result`] alias where the `Err` case is [`Error`].
pub type Result<T> = std::result::Result<T, Error>;

/// An error returned by the API.
///
/// This is an enum representing either an http error [`reqwest::Error`], or an error parsing the
/// output [`serde_xml_rs::Error`], or finally just a string. Which is typically returned in some
/// case an error shouldn't happen.
#[derive(Debug)]
pub enum Error {
    /// An error was returned making the HTTP request, or an error
    /// status code was returned.
    HttpError(reqwest::Error),
    /// The request tried too many times and timed out before the
    /// data was ready to be returned by the API. Includes the total
    /// number of times tried.
    MaxRetryError(u32),
    /// An error occurred attempting to parse the response from
    /// the API into the expected type.
    InvalidResponseError(serde_xml_rs::Error),
    /// A response was successfully retrieved and parsed from the underlying API but it wasn't what
    /// we expected.
    ///
    /// This error will be returned when we expect to get exactly one of something and the API
    /// returns multiple. Should never happen.
    UnexpectedResponseError(String),
    /// The username requested was not found.
    UnknownUsernameError,
    /// Invalid value supplied for subtype ([`crate::ItemType`]) query parameter.
    InvalidCollectionItemType,
    /// A generic not found error was returned from the underlying API.
    ///
    /// Note this is not a 404 returned from the request, rather a 200 but the content was an XML
    /// error tag containing the message "Not Found".
    ItemNotFound,
    /// The underlying API returned a list of errors that we do not recognise.
    UnknownApiErrors(Vec<String>),
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::HttpError(err)
    }
}

impl From<serde_xml_rs::Error> for Error {
    fn from(err: serde_xml_rs::Error) -> Self {
        Error::InvalidResponseError(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::HttpError(e) => write!(f, "error making request: {e}"),
            Error::MaxRetryError(retries) => {
                write!(f, "data still not ready after {retries} retries, aborting")
            },
            Error::InvalidResponseError(e) => write!(f, "error parsing output: {e}"),
            Error::UnexpectedResponseError(reason) => {
                write!(f, "unexpected response from API, {reason}")
            },
            Error::UnknownUsernameError => write!(f, "username not found"),
            Error::InvalidCollectionItemType => write!(f, "invalid collection item type provided"),
            Error::ItemNotFound => write!(f, "requested item not found"),
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
            Error::MaxRetryError(_) => None,
            Error::InvalidResponseError(e) => Some(e),
            Error::UnexpectedResponseError(_) => None,
            Error::UnknownUsernameError => None,
            Error::InvalidCollectionItemType => None,
            Error::ItemNotFound => None,
            Error::UnknownApiErrors(_) => None,
        }
    }
}

// Note this should be possible by making an enum of the three and adding
// `#[serde(untagged)]` and deriving deserialize, but it didn't work for
// some reason.
pub(crate) fn deserialize_maybe_error(api_response: &str) -> Option<Error> {
    let maybe_error_list = from_str::<ApiXmlErrorList>(api_response);
    if let Ok(error_list) = maybe_error_list {
        return Some(error_list.into());
    }
    let maybe_id_error = from_str::<IdApiXmlError>(api_response);
    if let Ok(id_error) = maybe_id_error {
        return Some(id_error.into());
    }
    let maybe_single_error = from_str::<SingleApiXmlError>(api_response);
    if let Ok(single_error) = maybe_single_error {
        return Some(single_error.into());
    }
    // Couldn't pass the API response as an error. This could mean the
    // response contained malformed data or was in an unexpected format.
    None
}

// The XML returned by the API in case of an error is a list
// of `message` tags. Usually with just one error inside.
#[derive(Debug, Deserialize)]
pub(crate) struct ApiXmlErrorList {
    #[serde(rename = "$value")]
    errors: Vec<ApiXmlError>,
}

#[derive(Debug, Deserialize)]
struct ApiXmlError {
    message: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct IdApiXmlError {
    // id: u64,
    error: String,
}

// For some endpoints an error can be returned in the form `<error message="Not Found"/>`,
// such as for guilds with a too high ID requested.
#[derive(Debug, Deserialize)]
pub(crate) struct SingleApiXmlError {
    pub(crate) message: String,
}

impl From<SingleApiXmlError> for Error {
    fn from(error: SingleApiXmlError) -> Self {
        if error.message == "Not Found" {
            return Self::ItemNotFound;
        }
        Self::UnknownApiErrors(vec![error.message])
    }
}

impl From<ApiXmlErrorList> for Error {
    fn from(error_list: ApiXmlErrorList) -> Self {
        let error_messages: Vec<String> =
            error_list.errors.into_iter().map(|e| e.message).collect();
        error_from_api_error_messages(error_messages)
    }
}

impl From<IdApiXmlError> for Error {
    fn from(error: IdApiXmlError) -> Self {
        if error.error == "Guild not found." {
            return Error::ItemNotFound;
        }
        Error::UnknownApiErrors(vec![error.error])
    }
}

fn error_from_api_error_messages(messages: Vec<String>) -> Error {
    if messages.len() == 1 {
        let error_message = &messages[0];
        if error_message == "Invalid username specified" {
            return Error::UnknownUsernameError;
        }
        if error_message == "Guild not found." {
            return Error::ItemNotFound;
        }
        if error_message == "Invalid collection subtype" {
            return Error::InvalidCollectionItemType;
        }
    }
    Error::UnknownApiErrors(messages)
}
