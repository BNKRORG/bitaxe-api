//! Antminer API errors

use thiserror::Error;

/// Antminer API error.
#[derive(Debug, Error)]
pub enum Error {
    /// HTTP request failed.
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    /// URL parsing or joining failed.
    #[error(transparent)]
    Url(#[from] url::ParseError),
    /// The miner requested authentication without returning a Digest challenge.
    #[error("missing Digest authentication challenge")]
    MissingAuthenticationChallenge,
    /// The miner returned an invalid Digest authentication challenge.
    #[error("invalid Digest authentication challenge")]
    InvalidAuthenticationChallenge,
    /// Digest authentication failed.
    #[error(transparent)]
    DigestAuthentication(#[from] digest_auth::Error),
    /// Hashrate must be a non-negative finite number
    #[error("hashrate must be a non-negative finite number")]
    InvalidHashrate,
    /// Hashrate is too large
    #[error("hashrate is too large")]
    HashrateTooLarge,
    /// Unknown miner mode
    #[error("unknown miner mode")]
    UnknownMinerMode,
}
