use std::{borrow::Cow, fmt};

use async_trait::async_trait;
use serde_json::Value;
use thiserror::Error;

use crate::AppClient;

#[derive(Debug, Error)]
pub struct HttpClientError {
    pub source: anyhow::Error,

    /// Response status code (if available)
    pub status_code: Option<u16>,
}

impl fmt::Display for HttpClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            source,
            status_code,
        } = self;
        if let Some(status_code) = status_code {
            write!(
                f,
                "HTTP client request failed with status {}: {}",
                status_code, source
            )
        } else {
            write!(f, "HTTP client request failed: {}", source)
        }
    }
}

#[async_trait]
pub trait HttpClient<'t> {
    type Error: Into<HttpClientError>;

    async fn fetch_token(&self, request: TokenRequest<'t>) -> Result<Value, Self::Error>;
}

#[derive(Debug)]
pub struct TokenRequest<'a> {
    auth_header: Header<'a>,
    content_type: Header<'static>,
    form_data: FormData<'a>,
}

impl<'a> TokenRequest<'a> {
    pub fn new(
        app_client: &AppClient,
        code: impl Into<Cow<'a, str>>,
        redirect_uri: impl Into<Cow<'a, str>>,
    ) -> Self {
        let value = base64::encode(&format!("{}:{}", app_client.id, app_client.secret));
        let auth_header = Header::new("Authorization", format!("Basic {}", value));
        let content_type = Header::new("Content-type", "application/x-www-form-urlencoded");
        let form_data = FormData {
            grant_type: "authorization_code",
            code: code.into(),
            redirect_uri: redirect_uri.into(),
        };
        Self {
            auth_header,
            form_data,
            content_type,
        }
    }
    pub const fn method(&self) -> &'static str {
        "POST"
    }
    pub const fn url(&self) -> &'static str {
        "https://accounts.spotify.com/api/token"
    }
    pub fn headers(&self) -> impl Iterator<Item = &Header> {
        [&self.auth_header, &self.content_type].into_iter()
    }
    pub const fn form_data(&self) -> &FormData {
        &self.form_data
    }
}

#[derive(Debug)]
pub struct Header<'a> {
    name: Cow<'a, str>,
    value: Cow<'a, str>,
}

impl<'a> Header<'a> {
    fn new(name: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
    pub fn name(&self) -> Cow<'a, str> {
        self.name.clone()
    }
    pub fn value(&self) -> Cow<'a, str> {
        self.value.clone()
    }
}

#[derive(Debug)]
pub struct FormData<'a> {
    grant_type: &'static str,
    code: Cow<'a, str>,
    redirect_uri: Cow<'a, str>,
}

impl<'a> FormData<'a> {
    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
        [
            ("grant_type", self.grant_type),
            ("code", &self.code),
            ("redirect_uri", &self.redirect_uri),
        ]
        .into_iter()
    }
}
