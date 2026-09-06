/// Centralized OIDC configuration.
///
/// All values are read from compile-time environment variables injected by
/// Trunk (or your CI/CD pipeline). This keeps OIDC details out of application
/// code and makes environment-specific overrides straightforward.
///
/// Set these in your Trunk.toml or CI environment:
///   DITA_OIDC_ISSUER          - e.g. https://auth.example.com
///   DITA_OIDC_CLIENT_ID       - the public client ID
///   DITA_OIDC_REDIRECT_URI    - e.g. https://app.example.com/
///   DITA_OIDC_POST_LOGOUT_URI - e.g. https://app.example.com/
use leptos_oidc2::{AuthParameters, Challenge};

pub fn oidc_parameters() -> AuthParameters {
    let app_path = option_env!("TRUNK_BUILD_PUBLIC_URL")
        .unwrap_or_default();
    AuthParameters {
        issuer: option_env!("DITA_OIDC_ISSUER")
            .unwrap_or("https://auth.outi.ir")
            .to_string(),
        client_id: option_env!("DITA_OIDC_CLIENT_ID")
            .unwrap_or("dita-web")
            .to_string(),
        redirect_uri: option_env!("DITA_OIDC_REDIRECT_URI")
            .unwrap_or(format!("http://localhost:8080{}/callback", app_path).as_str())
            .to_string(),
        post_logout_redirect_uri: option_env!("DITA_OIDC_POST_LOGOUT_URI")
            .unwrap_or(format!("http://localhost:8080{}/", app_path).as_str())
            .to_string(),
        // PKCE S256 is the required method for public clients (CSR/WASM)
        challenge: Challenge::S256,
        scope: Some("openid offline_access profile email".to_string()),
        audience: None,
    }
}
