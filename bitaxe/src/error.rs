//! Bitaxe API error

use thiserror::Error;

/// Bitaxe API error
#[derive(Debug, Error)]
pub enum Error {
    /// Reqwest error
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    /// Url error
    #[error(transparent)]
    Url(#[from] url::ParseError),
}
