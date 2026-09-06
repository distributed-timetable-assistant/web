use crate::config::oidc_parameters;
use crate::state_mgmt::StateValidationError;
/// Authentication mode hook following the DiTA shared-state architecture.
///
/// This mirrors the pattern used by `SidenavMode`, `ThemeMode`, and `HeaderMode`:
///   hooks/auth.rs → AuthState → StateProvider → AppState::builder().with::<AuthState>()
///
/// Authentication state is driven by `leptos_oidc2`'s `AuthSignal` (`RwSignal<Auth>`).
/// `AuthState::provide()` provisions the signal into the Leptos context so that
/// `Auth::init(...)` can pick it up, and also kicks off the OIDC initialization.
///
/// Applications call `AuthState::new()` inside components to read state and
/// perform login/logout – without ever touching OIDC internals directly.
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
    /// The user explicitly canceled the login flow.
    /// Show a retry UI rather than immediately redirecting.
    LoginCanceled,
    /// An OAuth2 state validation failure occurred (e.g. state missing or mismatch).
    StateError(StateValidationError),
    /// An OIDC error occurred (e.g. token fetch failed, provider error).
    Error(AuthError),
}

/// The `AuthState` hook – the DiTA façade over `leptos_oidc2`.
#[derive(Clone, Copy)]
pub struct AuthState {
    signal: AuthSignal,
    /// Tracks whether the user explicitly canceled a login attempt.
    canceled: RwSignal<bool>,
    /// Tracks whether an OAuth2 state validation error occurred during callback.
    state_error: RwSignal<Option<StateValidationError>>,
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
        let canceled = use_context::<RwSignal<bool>>().expect(
            "AuthState: login-canceled signal not found. \
             Did you call AppState::builder().with::<AuthState>() at the app root?",
        );
        let state_error = use_context::<RwSignal<Option<StateValidationError>>>().expect(
            "AuthState: state_error signal not found. \
             Did you call AppState::builder().with::<AuthState>() at the app root?",
        );
        Self {
            signal,
            canceled,
            state_error,
        }
    }

    /// Returns the current authentication status as a `Signal<AuthStatus>`.
    ///
    /// Derives `LoginCancelled` and `StateError` - keeping the state clean and reactive.
    pub fn status(&self) -> Signal<AuthStatus> {
        let signal = self.signal;
        let canceled = self.canceled;
        let state_error = self.state_error;
        Signal::derive(move || {
            if let Some(err) = state_error.get() {
                return AuthStatus::StateError(err);
            }

            match signal.get() {
                Auth::Loading => AuthStatus::Loading,
                Auth::Authenticated(_) => AuthStatus::Authenticated,
                Auth::Unauthenticated(_) => {
                    if canceled.get() {
                        AuthStatus::LoginCanceled
                    } else {
                        AuthStatus::Unauthenticated
                    }
                }
                Auth::Error(e) => AuthStatus::Error(e),
            }
        })
    }

    /// Returns `true` if the user is fully authenticated.
    pub fn is_authenticated(&self) -> Signal<bool> {
        let signal = self.signal;
        Signal::derive(move || signal.get().is_authenticated())
    }

    /// Generates a cryptographically random OAuth2 state token, stores it in session storage,
    /// appends it to the authorization URL, and redirects the browser to the OIDC provider.
    ///
    /// Only has an effect while the auth state is `Unauthenticated`.
    /// Clears any previous canceled or state error flags.
    pub fn login(&self) {
        let signal = self.signal;
        let canceled = self.canceled;
        let state_error = self.state_error;

        let login_url = signal
            .with(Auth::unauthenticated)
            .map(|unauth| unauth.login_url())
            .map(|login_url| Url::from_str(&login_url))
            .transpose()
            .ok()
            .flatten();
        if let Some(url) = login_url {
            canceled.set(false);
            state_error.set(None);

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

    /// Mark the login flow as canceled.
    pub fn mark_canceled(&self) {
        self.canceled.set(true);
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
    /// Called once at application startup via `AppState::builder().with::<AuthState>()`.
    ///
    /// Provisions:
    ///  1. The `AuthSignal` that `Auth::init(...)` requires in context.
    ///  2. A `RwSignal<bool>` tracking explicit login cancellation.
    ///  3. A `RwSignal<Option<StateValidationError>>` tracking OAuth2 state validation.
    ///  5. The `Auth::init(...)` call itself.
    fn provide() {
        // 1. Create and provide the AuthSignal FIRST – Auth::init reads it from context.
        let auth_signal = Auth::signal();
        provide_context(auth_signal);

        // 2. Provide the cancellation tracking signal.
        let canceled: RwSignal<bool> = RwSignal::new(false);
        provide_context(canceled);

        // 3. Provide the state validation error signal.
        let state_error: RwSignal<Option<StateValidationError>> = RwSignal::new(None);
        provide_context(state_error);

        // 5. Kick off OIDC initialization.
        let _awaitable = Auth::init(oidc_parameters());
    }
}
