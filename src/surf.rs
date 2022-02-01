use crate::{error::*, HttpClient, TokenRequest};
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::Value;
use snafu::ResultExt;
use std::collections::HashMap;
use surf::Body;

pub struct SurfClient;

#[derive(Deserialize)]
struct Error {
    error: String,
}

#[async_trait]
impl<'t> HttpClient<'t> for SurfClient {
    type Error = surf::Error;
    async fn fetch_token(&self, auth_request: TokenRequest<'t>) -> Result<Value, Self::Error> {
        // POST the request.
        let mut request = surf::post(auth_request.url());
        for h in auth_request.headers() {
            request = request.header(&*h.name(), h.value());
        }
        let form_data = auth_request.form_data().iter().collect::<HashMap<_, _>>();
        request = request.body(Body::from_form(&form_data)?);
        let mut response = request.send().await?;
        let json_string = response.body_string().await?;
        if !response.status().is_success() {
            let err: Error = serde_json::from_str(&json_string).context(SerdeError)?;
            return Err(surf::Error::new(
                response.status(),
                anyhow::anyhow!("Failed to fetch token: {}", err.error),
            ));
        }
        let value = serde_json::from_str(&json_string).context(SerdeError)?;
        Ok(value)
    }
}
