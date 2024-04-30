use std::{fmt::Display, path::PathBuf};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Serde(serde_json::Error),
    Xml(quick_xml::Error),
    XmlTidy(String),
    XmlOutPut(String),
    InputDir(PathBuf),
    InputFile(PathBuf),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Serde(err)
    }
}

impl From<quick_xml::Error> for Error {
    fn from(err: quick_xml::Error) -> Self {
        Error::Xml(err)
    }
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Error::XmlTidy(err)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Error::XmlTidy(err.to_string())
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(err: std::str::Utf8Error) -> Self {
        Error::XmlTidy(err.to_string())
    }
}

impl From<quick_xml::DeError> for Error {
    fn from(err: quick_xml::DeError) -> Self {
        Error::XmlOutPut(err.to_string())
    }
}

// region:		--- Boilerplate
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Io(err) => write!(f, "IO Error: {}", err),
            Error::Serde(err) => write!(f, "Serde Error: {}", err),
            Error::Xml(_) => write!(f, "Xml Error"),
            Error::XmlTidy(_) => write!(f, "Xml Tidy Error"),
            Error::XmlOutPut(e) => write!(f, "Xml Output Error: {}", e),
            Error::InputDir(_) => write!(f, "Input Directory Error"),
            Error::InputFile(_) => write!(f, "Input File Error, doesn't exist, or corrupt"),
        }
    }
}

impl std::error::Error for Error {}
// endregion:	--- Boilerplate
