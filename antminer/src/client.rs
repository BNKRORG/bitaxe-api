//! Antminer API client

use std::borrow::Cow;
use std::fmt;
use std::time::Duration;

use digest_auth::{AuthContext, WwwAuthenticateHeader};
use reqwest::header::{AUTHORIZATION, WWW_AUTHENTICATE};
use reqwest::{Client, Method, Response, StatusCode};
use serde::de::DeserializeOwned;
use url::Url;

use crate::constant::USER_AGENT_NAME;
use crate::error::Error;
use crate::response::{MinerType, PoolsResponse, StatsResponse, SummaryResponse, SystemInfo};

/// Client for the read-only Antminer CGI API.
#[derive(Clone)]
pub struct AntminerClient {
    root_url: Url,
    username: String,
    password: String,
    client: Client,
}

impl fmt::Debug for AntminerClient {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AntminerClient")
            .field("root_url", &self.root_url)
            .field("username", &self.username)
            .field("password", &"[redacted]")
            .field("client", &self.client)
            .finish()
    }
}

impl AntminerClient {
    /// Creates an Antminer client using the web interface credentials.
    pub fn new<U, P>(root_url: Url, username: U, password: P) -> Result<Self, Error>
    where
        U: Into<String>,
        P: Into<String>,
    {
        Ok(Self {
            root_url,
            username: username.into(),
            password: password.into(),
            client: Client::builder()
                .user_agent(USER_AGENT_NAME)
                .timeout(Duration::from_secs(25))
                .build()?,
        })
    }

    async fn request<T>(&self, method: Method, path: &str) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let url: Url = self.root_url.join(path)?;

        let response: Response = self
            .client
            .request(method.clone(), url.clone())
            .send()
            .await?;

        let response: Response = if response.status() == StatusCode::UNAUTHORIZED {
            self.retry_with_digest_authentication(method, url, response)
                .await?
        } else {
            response
        };

        let response: Response = response.error_for_status()?;

        Ok(response.json().await?)
    }

    async fn retry_with_digest_authentication(
        &self,
        method: Method,
        url: Url,
        response: Response,
    ) -> Result<Response, Error> {
        let challenge: &str = response
            .headers()
            .get(WWW_AUTHENTICATE)
            .ok_or(Error::MissingAuthenticationChallenge)?
            .to_str()
            .map_err(|_| Error::InvalidAuthenticationChallenge)?;

        let mut prompt: WwwAuthenticateHeader = digest_auth::parse(challenge)?;

        let request_target: Cow<str> = match url.query() {
            Some(query) => Cow::Owned(format!("{}?{query}", url.path())),
            None => Cow::Borrowed(url.path()),
        };
        let context: AuthContext = AuthContext::new(&self.username, &self.password, request_target);
        let authorization: String = prompt.respond(&context)?.to_string();

        Ok(self
            .client
            .request(method, url)
            .header(AUTHORIZATION, authorization)
            .send()
            .await?)
    }

    /// Gets system and network information.
    ///
    /// Endpoint: `GET /cgi-bin/get_system_info.cgi`.
    ///
    /// See the [Bitmain API documentation][system-info-docs].
    ///
    /// [system-info-docs]: https://docs.bitmain.com/en/antminer/CONTENTS.html#get_system_info
    #[inline]
    pub async fn system_info(&self) -> Result<SystemInfo, Error> {
        self.request(Method::GET, "/cgi-bin/get_system_info.cgi")
            .await
    }

    /// Gets the miner model and firmware build information.
    ///
    /// Endpoint: `GET /cgi-bin/miner_type.cgi`.
    ///
    /// See the [Bitmain API documentation][miner-type-docs].
    ///
    /// [miner-type-docs]: https://docs.bitmain.com/en/antminer/CONTENTS.html#miner_type
    #[inline]
    pub async fn miner_type(&self) -> Result<MinerType, Error> {
        self.request(Method::GET, "/cgi-bin/miner_type.cgi").await
    }

    /// Gets the high-level mining summary.
    ///
    /// Endpoint: `GET /cgi-bin/summary.cgi`.
    ///
    /// See the [Bitmain API documentation][summary-docs].
    ///
    /// [summary-docs]: https://docs.bitmain.com/en/antminer/CONTENTS.html#summary
    #[inline]
    pub async fn summary(&self) -> Result<SummaryResponse, Error> {
        self.request(Method::GET, "/cgi-bin/summary.cgi").await
    }

    /// Gets detailed miner, fan, and hashboard statistics.
    ///
    /// Endpoint: `GET /cgi-bin/stats.cgi`.
    ///
    /// See the [Bitmain API documentation][stats-docs].
    ///
    /// [stats-docs]: https://docs.bitmain.com/en/antminer/CONTENTS.html#stats
    #[inline]
    pub async fn stats(&self) -> Result<StatsResponse, Error> {
        self.request(Method::GET, "/cgi-bin/stats.cgi").await
    }

    /// Gets the configured pools and their mining statistics.
    ///
    /// Endpoint: `GET /cgi-bin/pools.cgi`.
    ///
    /// See the [Bitmain API documentation][pools-docs].
    ///
    /// [pools-docs]: https://docs.bitmain.com/en/antminer/CONTENTS.html#pools
    #[inline]
    pub async fn pools(&self) -> Result<PoolsResponse, Error> {
        self.request(Method::GET, "/cgi-bin/pools.cgi").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_output_redacts_password() {
        let url = Url::parse("http://192.168.1.100").unwrap();
        let client = AntminerClient::new(url, "root", "secret-password").unwrap();

        let output = format!("{client:?}");

        assert!(output.contains("[redacted]"));
        assert!(!output.contains("secret-password"));
    }
}
