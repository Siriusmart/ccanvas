use std::{error, fmt};

use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum Error {
    #[serde(rename = "unspecified")]
    Unspecified,

    #[serde(rename = "unsupported event")]
    UnsupportedEvent(Vec<u8>),

    #[serde(rename = "unsupported key")]
    UnsupportedKey,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unspecified => f.write_str("unspecified error"),
            Self::UnsupportedEvent(bytes) => {
                f.write_fmt(format_args!("unsupported event {bytes:?}"))
            }
            Self::UnsupportedKey => f.write_str("unsupported key"),
        }
    }
}

impl error::Error for Error {}
