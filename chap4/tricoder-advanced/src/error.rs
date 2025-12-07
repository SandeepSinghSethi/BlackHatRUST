use thiserror::Error;

#[derive(Debug,Clone,Error)]
pub enum Error{
    #[error("Usage: tricoder <google.com>")]
    CliUsage,

    #[error("Reqwest : {0}")]
    Reqwest(String),

    #[error("Tokio join error: {0}")]
    TokioJoinError(String),

    #[error("{0}: Invalid HTTP response")]
    InvalidHTTPResponse(String),
}

// any error of reqwest modules will be handled from here , we are implementing a proxy for errors of crate reqwest
impl std::convert::From<reqwest::Error> for Error{
    fn from(err: reqwest::Error) -> Self {
        Error::Reqwest(err.to_string())
    }
}

//likewise for tokiojoinerror
impl std::convert::From<tokio::task::JoinError> for Error{
    fn from(err: tokio::task::JoinError) -> Self {
        Error::TokioJoinError(err.to_string())
    }
}