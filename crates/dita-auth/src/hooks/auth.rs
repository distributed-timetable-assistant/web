use crate::config::oidc_parameters;
use dita_state::app_state::StateProvider;
use leptos::prelude::*;
use leptos_oidc2::{Auth, AuthError, AuthSignal};
use std::str::FromStr;
use url::Url;

/// Represents the resolved authentication state visible to application code.
/// Distinct from `leptos_oidc2::Auth` to add the `LoginCancelled` and `StateValidation` variants
/// and to avoid leaking OIDC types into application components.
#[derive(Clone, Debug)]
pub enum AuthStatus {
    /// OIDC state is still being loaded; do not redirect.
    Loading,
    /// The user is fully authenticated with a valid session.
    Authenticated,
    /// The user is not authenticated and has not yet attempted to log in.
    Unauthenticated,
    /// An OIDC error occurred (e.g. token fetch failed, provider error).
    Error(AuthError),
}

/// The `AuthState` hook – the DiTA façade over `leptos_oidc2`.
#[derive(Clone, Copy)]
pub struct AuthState {
    signal: AuthSignal,
}

impl AuthState {
    /// Obtain the `AuthState` hook from the Leptos context.
    ///
    /// Panics with a helpful message if `AuthState::provide()` was not called
    /// at a root component (i.e. `AppState::builder().with::<AuthState>()` was
    /// not invoked).
    pub fn new() -> Self {
        let signal = use_context::<AuthSignal>().expect(
            "AuthState: AuthSignal not found in context. \
             Did you call AppState::builder().with::<AuthState>() at the app root?",
        );
        Self { signal }
    }

    pub fn status(&self) -> Signal<AuthStatus> {
        let signal = self.signal;
        Signal::derive(move || match signal.get() {
            Auth::Loading => AuthStatus::Loading,
            Auth::Authenticated(_) => AuthStatus::Authenticated,
            Auth::Unauthenticated(_) => AuthStatus::Unauthenticated,
            Auth::Error(e) => AuthStatus::Error(e),
        })
    }

    /// Returns `true` if the user is fully authenticated.
    pub fn is_authenticated(&self) -> Signal<bool> {
        let signal = self.signal;
        Signal::derive(move || signal.get().is_authenticated())
    }

    pub fn login(&self) {
        let signal = self.signal;

        let login_url = signal
            .with(Auth::unauthenticated)
            .map(|unauth| unauth.login_url())
            .map(|login_url| Url::from_str(&login_url))
            .transpose()
            .ok()
            .flatten();
        if let Some(url) = login_url {
            let window = web_sys::window().expect("no window");
            let _ = window.location().set_href(&url.to_string());
        }
    }

    /// Redirects the browser to the OIDC provider's end-session endpoint.
    ///
    /// Only has an effect while the auth state is `Authenticated`.
    pub fn logout(&self) {
        let signal = self.signal;
        let url = signal.with(|auth| auth.authenticated().map(|authed| authed.logout_url()));
        if let Some(url) = url {
            let window = web_sys::window().expect("no window");
            let _ = window.location().set_href(&url);
        }
    }

    /// Returns the current access token, if authenticated.
    ///
    /// Intentionally not public to application code; used only by `DitaClient`
    /// inside `dita-auth` to inject Authorization headers.
    pub(crate) fn access_token(&self) -> Option<String> {
        self.signal
            .with(|auth| auth.authenticated().map(|a| a.access_token()))
    }
}

impl StateProvider for AuthState {
    fn provide() {
        let auth_signal = Auth::signal();
        provide_context(auth_signal);

        let _awaitable = Auth::init(oidc_parameters());
    }
}
