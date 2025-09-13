use crate::utils::error::{EtlError, Result};
use crate::config::settings::{AuthConfig, AuthType, RetryConfig};
use backoff::{ExponentialBackoff, future::retry};
use reqwest::{Client, Response, Method, header::{HeaderMap, HeaderName, HeaderValue}};
use std::collections::HashMap;
use std::time::Duration;
use std::str::FromStr;

pub struct ApiClient {
    client: Client,
    default_timeout: Duration,
}

impl ApiClient {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            default_timeout: Duration::from_secs(30),
        }
    }

    pub async fn fetch_with_config(
        &self,
        url: &str,
        method: Option<String>,
        headers: Option<HashMap<String, String>>,
        auth: Option<AuthConfig>,
        retry_config: Option<RetryConfig>,
    ) -> Result<Response> {
        let method = method
            .as_ref()
            .map(|m| Method::from_str(m))
            .transpose()
            .map_err(|e| EtlError::ConfigError(format!("Invalid HTTP method: {}", e)))?
            .unwrap_or(Method::GET);

        let mut request_builder = self.client.request(method, url);

        // 添加自定義標頭
        if let Some(headers) = headers {
            let mut header_map = HeaderMap::new();
            for (key, value) in headers {
                let header_name = HeaderName::from_str(&key)
                    .map_err(|e| EtlError::ConfigError(format!("Invalid header name '{}': {}", key, e)))?;
                let header_value = HeaderValue::from_str(&value)
                    .map_err(|e| EtlError::ConfigError(format!("Invalid header value '{}': {}", value, e)))?;
                header_map.insert(header_name, header_value);
            }
            request_builder = request_builder.headers(header_map);
        }

        // 添加認證
        if let Some(auth_config) = auth {
            request_builder = self.apply_auth(request_builder, auth_config)?;
        }

        let backoff = if let Some(config) = retry_config {
            ExponentialBackoff {
                current_interval: Duration::from_millis(config.initial_delay_ms),
                max_elapsed_time: Some(Duration::from_millis(config.max_delay_ms * config.max_attempts as u64)),
                ..Default::default()
            }
        } else {
            ExponentialBackoff::default()
        };

        let response = retry(backoff, || async {
            let request = request_builder
                .try_clone()
                .ok_or_else(|| EtlError::RequestError("Failed to clone request".to_string()))?;
            
            request
                .send()
                .await
                .map_err(|e| {
                    if e.is_timeout() || e.is_connect() {
                        backoff::Error::transient(e.into())
                    } else {
                        backoff::Error::permanent(e.into())
                    }
                })
        })
        .await?;

        if !response.status().is_success() {
            return Err(EtlError::HttpError(
                response.status().as_u16(),
                format!("HTTP request failed: {}", response.status()),
            ));
        }

        Ok(response)
    }

    pub async fn fetch_json(
        &self,
        url: &str,
        method: Option<String>,
        headers: Option<HashMap<String, String>>,
        auth: Option<AuthConfig>,
        retry_config: Option<RetryConfig>,
    ) -> Result<serde_json::Value> {
        let response = self.fetch_with_config(url, method, headers, auth, retry_config).await?;
        let json = response.json().await?;
        Ok(json)
    }

    pub async fn fetch_text(
        &self,
        url: &str,
        method: Option<String>,
        headers: Option<HashMap<String, String>>,
        auth: Option<AuthConfig>,
        retry_config: Option<RetryConfig>,
    ) -> Result<String> {
        let response = self.fetch_with_config(url, method, headers, auth, retry_config).await?;
        let text = response.text().await?;
        Ok(text)
    }

    pub async fn fetch_bytes(
        &self,
        url: &str,
        method: Option<String>,
        headers: Option<HashMap<String, String>>,
        auth: Option<AuthConfig>,
        retry_config: Option<RetryConfig>,
    ) -> Result<Vec<u8>> {
        let response = self.fetch_with_config(url, method, headers, auth, retry_config).await?;
        let bytes = response.bytes().await?;
        Ok(bytes.to_vec())
    }

    fn apply_auth(
        &self,
        mut request_builder: reqwest::RequestBuilder,
        auth_config: AuthConfig,
    ) -> Result<reqwest::RequestBuilder> {
        match auth_config.auth_type {
            AuthType::BasicAuth => {
                if let (Some(username), Some(password)) = (
                    auth_config.credentials.username,
                    auth_config.credentials.password,
                ) {
                    request_builder = request_builder.basic_auth(username, Some(password));
                } else {
                    return Err(EtlError::ConfigError(
                        "Basic auth requires username and password".to_string(),
                    ));
                }
            }
            AuthType::BearerToken => {
                if let Some(token) = auth_config.credentials.token {
                    request_builder = request_builder.bearer_auth(token);
                } else {
                    return Err(EtlError::ConfigError(
                        "Bearer auth requires token".to_string(),
                    ));
                }
            }
            AuthType::ApiKey => {
                if let (Some(api_key), Some(header_name)) = (
                    auth_config.credentials.api_key,
                    auth_config.credentials.header_name,
                ) {
                    request_builder = request_builder.header(header_name, api_key);
                } else {
                    return Err(EtlError::ConfigError(
                        "API key auth requires api_key and header_name".to_string(),
                    ));
                }
            }
            AuthType::OAuth2 => {
                return Err(EtlError::ConfigError(
                    "OAuth2 authentication not yet implemented".to_string(),
                ));
            }
        }

        Ok(request_builder)
    }
}
