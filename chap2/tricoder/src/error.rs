use thiserror::Error;
// use serde_json;

// using this derive is required just because rust by default dont have any implemenations for these structs or enums, so with #[derive(...functionalities...)] , we can give them functionality , like println #[derive(Display)] or debug {:?}/{:#?} with #[derive(Debug)]

#[derive(Error,Debug,Clone)] // it allows this enum to have Error,debug and clone functions 
pub enum Error {
    #[error("Usage: tricoder <domain-name.com>")] // <- thiserror::Error utilizes this trait
    CliUsage,
    #[error("Reqwest: {0}")] // <- this as well
    Reqwest(String),
    // #[error("Serde_Json: {0}")]
    // Serde(String),
}

// what we are doing in this is we are converting the reqwest error type to Error type , with impl , as it implements a translation for errors of reqwest type..

impl std::convert::From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Reqwest(err.to_string())
    }
}

// impl std::convert::From<serde_json::Error> for Error {
//     fn from(value: serde_json::Error) -> Self {
//         Error::Serde(value.to_string())
//     }
// }