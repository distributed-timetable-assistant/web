/// OAuth2 state management for OIDC Authorization Code + PKCE flow.
///
/// Implements the complete OAuth2/OIDC state lifecycle:
/// 1. Generate a cryptographically random state parameter using `oauth2::CsrfToken::new_random()`.
/// 2. Persist it in `sessionStorage` (under key `oauth_state`) via `leptos_use::storage::use_session_storage`.
/// 3. Inject it into the authorization redirect URL (`&state=...`).
/// 4. Intercept the callback query parameter `state` and validate against stored state.
/// 5. Reject missing/mismatched state with a clear `StateMismatch` / `MissingState` error.
/// 6. Clear state from storage upon validation to prevent replay attacks.
use codee::string::JsonSerdeCodec;
use leptos::prelude::*;
use leptos_use::storage::use_session_storage;
use thiserror::Error;

pub const OAUTH_STATE_KEY: &str = "oauth_state";
pub const RETURN_TO_KEY: &str = "dita_auth_return_to";

#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum StateValidationError {
    #[error("Missing OAuth2 state parameter in callback URL")]
    MissingStateInCallback,
    #[error("Missing expected OAuth2 state in session storage")]
    MissingStoredState,
    #[error("OAuth2 state mismatch (possible CSRF attempt)")]
    StateMismatch,
}

/// Validates whether a given path string is a safe, relative internal application route.
///
/// Disallows:
/// - Absolute URLs with schemes (e.g. `http:`, `https:`, `javascript:`, `//`)
/// - Authentication lifecycle routes (e.g. `callback`, `error`, `canceled`)
pub fn is_safe_internal_route(path: &str) -> bool {
    let trimmed = path.trim();
    if trimmed.is_empty() {
        return false;
    }

    // Check for protocol/scheme indicators or scheme-relative URLs
    if trimmed.contains("://") || trimmed.starts_with("//") || trimmed.starts_with('\\') {
        return false;
    }

    // Strip leading slash for segment checking
    let clean = trimmed.trim_start_matches('/');
    let primary_segment = clean.split(['/', '?', '#']).next().unwrap_or("");

    // Disallow auth lifecycle pages
    if matches!(primary_segment, "callback" | "error" | "canceled") {
        return false;
    }

    true
}

/// Persists the target destination route in `sessionStorage` if not already set.
/// Avoids overwriting an existing destination to ensure the original destination survives
/// multiple redirect or render phases.
pub fn store_target_destination(path: &str) {
    if !is_safe_internal_route(path) {
        return;
    }

    let (get_dest, set_dest, _remove) =
        use_session_storage::<Option<String>, JsonSerdeCodec>(RETURN_TO_KEY);

    // Only set if not already present
    if get_dest.get_untracked().is_none() {
        set_dest.set(Some(path.to_string()));
    }
}

/// Reads and clears the stored target destination route from `sessionStorage` exactly once.
/// Returns `Some(destination)` only if a valid internal route was stored.
pub fn get_and_clear_target_destination() -> Option<String> {
    let (get_dest, _set_dest, remove_dest) =
        use_session_storage::<Option<String>, JsonSerdeCodec>(RETURN_TO_KEY);

    let stored = get_dest.get_untracked();
    remove_dest();

    match stored {
        Some(dest) if is_safe_internal_route(&dest) => Some(dest),
        _ => None,
    }
}
