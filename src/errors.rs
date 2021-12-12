use serde_json::error as serde_err;
use std::{cmp, env, error, fmt, io, result};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    TokenPathError,
    HomeDirError,
    IOError(io::Error),
    JSONError(serde_err::Error),
    EnvVarError(env::VarError),
    ReqwestError(reqwest::Error),
    UrlError(url::ParseError),
    RedirectUriCfgError,
    UserError(Box<dyn error::Error>),
    RefreshTokenValue,
    TokenRequestError(String),
}

unsafe impl Send for Error {}
unsafe impl Sync for Error {}

impl cmp::PartialEq for Error {
    fn eq(&self, other: &Error) -> bool {
        match (&self, other) {
            (Error::TokenPathError, Error::TokenPathError) => true,
            (Error::HomeDirError, Error::HomeDirError) => true,
            (Error::IOError(_), Error::IOError(_)) => true,
            (Error::JSONError(_), Error::JSONError(_)) => true,
            (Error::EnvVarError(_), Error::EnvVarError(_)) => true,
            (Error::ReqwestError(_), Error::ReqwestError(_)) => true,
            (Error::UrlError(_), Error::UrlError(_)) => true,
            (Error::RedirectUriCfgError, Error::RedirectUriCfgError) => true,
            (Error::UserError(_), Error::UserError(_)) => true,
            (Error::RefreshTokenValue, Error::RefreshTokenValue) => true,
            (Error::TokenRequestError(s1), Error::TokenRequestError(s2)) => s1 == s2,
            (_, _) => false,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // todo: get rid of duplication the info here and in self.description()
            Error::TokenPathError => write!(f, "failed to retrieve token dir from filekey"),
            Error::IOError(ref e) => e.fmt(f),
            Error::JSONError(ref e) => e.fmt(f),
            Error::EnvVarError(ref e) => e.fmt(f),
            Error::ReqwestError(ref e) => e.fmt(f),
            Error::UrlError(ref e) => e.fmt(f),
            Error::HomeDirError => write!(f, "failed to identify home directory"),
            Error::RedirectUriCfgError => {
                write!(f, "failed to retrieve redirect_uri from credentials")
            }
            Error::UserError(ref e) => e.fmt(f),
            Error::RefreshTokenValue => {
                write!(f, "expected a refresh token string value, got None")
            }
            Error::TokenRequestError(ref s) => {
                write!(f, "oauth server returned error: {}", s)
            }
        }
    }
}

impl error::Error for Error {}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IOError(err)
    }
}

impl From<serde_err::Error> for Error {
    fn from(err: serde_err::Error) -> Error {
        Error::JSONError(err)
    }
}

impl From<env::VarError> for Error {
    fn from(err: env::VarError) -> Error {
        Error::EnvVarError(err)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::ReqwestError(err)
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Error {
        Error::UrlError(err)
    }
}
