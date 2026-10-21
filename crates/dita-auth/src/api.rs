/// Minimal WASM/CSR HTTP API client for DiTA.
///
/// Encapsulates authentication details and automatically injects
/// `Authorization: Bearer <access_token>` into outbound requests.
/// Application code never touches raw access tokens.
///
/// Uses `gloo-net` for browser-native Fetch API calls.
use gloo_net::http::{Request, Response};
use thiserror::Error;

use crate::hooks::auth::AuthState;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Network / HTTP error: {0}")]
    Gloo(#[from] gloo_net::Error),
    #[error("Unauthorized (401): Session expired or invalid")]
    Unauthorized,
    #[error("HTTP Error {status}: {message}")]
    HttpStatus { status: u16, message: String },
}

#[derive(Clone, Copy)]
pub struct DitaClient {
    auth: AuthState,
}

impl DitaClient {
    /// Creates a new `DitaClient` instance bound to the current authentication context.
    pub fn new() -> Self {
        Self {
            auth: AuthState::new(),
        }
    }

    /// Performs an authenticated GET request.
    pub async fn get(&self, url: &str) -> Result<Response, ApiError> {
        let req = Request::get(url);
        self.send(req).await
    }

    /// Performs an authenticated POST request with a JSON body.
    pub async fn post<T: serde::Serialize>(
        &self,
        url: &str,
        body: &T,
    ) -> Result<Response, ApiError> {
        let req = Request::post(url).json(body)?;
        self.send_built(req).await
    }

    /// Performs an authenticated PUT request with a JSON body.
    pub async fn put<T: serde::Serialize>(
        &self,
        url: &str,
        body: &T,
    ) -> Result<Response, ApiError> {
        let req = Request::put(url).json(body)?;
        self.send_built(req).await
    }

    /// Performs an authenticated DELETE request.
    pub async fn delete(&self, url: &str) -> Result<Response, ApiError> {
        let req = Request::delete(url);
        self.send(req).await
    }

    /// Helper that attaches the Bearer token and executes a RequestBuilder.
    async fn send(
        &self,
        mut builder: gloo_net::http::RequestBuilder,
    ) -> Result<Response, ApiError> {
        if let Some(token) = self.auth.access_token() {
            builder = builder.header("Authorization", &format!("Bearer {}", token));
        }

        let resp = builder.send().await?;
        self.check_response(resp)
    }

    /// Helper that attaches the Bearer token and executes a built Request.
    async fn send_built(&self, req: Request) -> Result<Response, ApiError> {
        if let Some(token) = self.auth.access_token() {
            req.headers()
                .set("Authorization", &format!("Bearer {}", token));
        }

        let resp = req.send().await?;
        self.check_response(resp)
    }

    fn check_response(&self, resp: Response) -> Result<Response, ApiError> {
        if resp.status() == 401 {
            // Handle 401 Unauthorized appropriately without creating redirect loops
            return Err(ApiError::Unauthorized);
        }

        if !resp.ok() {
            return Err(ApiError::HttpStatus {
                status: resp.status(),
                message: resp.status_text(),
            });
        }

        Ok(resp)
    }
}
