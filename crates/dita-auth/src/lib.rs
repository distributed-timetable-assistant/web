pub mod api;
pub mod components;
pub mod config;
pub mod hooks;
pub mod pages;
pub mod state_mgmt;

// Public re-exports for application code
pub use api::{ApiError, DitaClient};
pub use components::guard::RequireAuth;
pub use hooks::auth::{AuthState, AuthStatus};
pub use pages::{AuthCallbackPage, AuthCancelledPage, AuthErrorPage, AuthLoadingPage};
pub use state_mgmt::StateValidationError;
