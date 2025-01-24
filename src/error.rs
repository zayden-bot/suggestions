use zayden_core::Error as ZaydenError;
use zayden_core::ErrorResponse;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    MissingGuildId,
    MissingSuggesionChannel,
}

impl ErrorResponse for Error {
    fn to_response(&self) -> &str {
        match self {
            Error::MissingGuildId => ZaydenError::MissingGuildId.to_response(),
            Error::MissingSuggesionChannel => "Please specify a channel to fetch suggestions from.",
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}
