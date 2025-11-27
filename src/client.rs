//! Bitaxe API client

use std::time::Duration;

use reqwest::{Client, Method, Response};
use serde::de::DeserializeOwned;
use url::Url;

use crate::constant::USER_AGENT_NAME;
use crate::error::Error;
use crate::response::SystemInfo;

/// Bitaxe client
#[derive(Debug, Clone)]
pub struct BitaxeClient {
    /// Root URL for the API.
    root_url: Url,
    /// HTTP client.
    client: Client,
}

impl BitaxeClient {
    /// Create a new Bitaxe client.
    pub fn new(root_url: Url) -> Result<Self, Error> {
        Ok(Self {
            root_url,
            client: Client::builder()
                .user_agent(USER_AGENT_NAME)
                .timeout(Duration::from_secs(25))
                .build()?,
        })
    }

    async fn request<T>(&self, method: Method, url: Url) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        // Post request
        let response: Response = self.client.request(method, url).send().await?;

        // If HTTP error, return error
        let response: Response = response.error_for_status()?;

        // Parse the response as JSON
        Ok(response.json().await?)
    }

    /// Get system information
    #[inline]
    pub async fn system_info(&self) -> Result<SystemInfo, Error> {
        let url: Url = self.root_url.join("/api/system/info")?;
        self.request(Method::GET, url).await
    }
}
