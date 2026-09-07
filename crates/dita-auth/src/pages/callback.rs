use leptos::prelude::*;
use leptos_router::NavigateOptions;
use leptos_router::hooks::use_navigate;

use crate::hooks::auth::{AuthState, AuthStatus};
use crate::pages::loading::AuthLoadingPage;
use crate::state_mgmt::get_and_clear_target_destination;

/// Dedicated authentication callback page.
///
/// Observes the authentication result produced by `leptos_oidc2` / `AuthState`.
/// Does not implement OIDC processing, parameter parsing, or token exchange directly.
/// Navigates to the preserved internal route or fallback destination upon successful authentication.
#[component]
pub fn AuthCallbackPage(
    #[prop(into, optional, default = "".to_string())] destination: String,
) -> impl IntoView {
    let auth = AuthState::new();
    let status = auth.status();
    let navigate = use_navigate();

    Effect::new(move |_| {
        let current_status = status.get();
        match current_status {
            AuthStatus::Authenticated => to_continue(&destination, &navigate),
            AuthStatus::Error(_) => error(&navigate),
            AuthStatus::Loading | AuthStatus::Unauthenticated => {
                // Stay in-place showing the loading UI while waiting for OIDC resolution
            }
        }
    });

    view! {
        <AuthLoadingPage />
    }
}

fn error(navigate: &(impl Fn(&str, NavigateOptions) + Clone)) {
    navigate(
        "../error",
        NavigateOptions {
            resolve: true,
            replace: true,
            ..Default::default()
        },
    );
}

fn to_continue(destination: &String, navigate: &(impl Fn(&str, NavigateOptions) + Clone)) {
    let target = get_and_clear_target_destination().unwrap_or_else(|| {
        if destination.is_empty() {
            "".to_string()
        } else {
            destination.clone()
        }
    });

    navigate(
        &target,
        NavigateOptions {
            resolve: false,
            replace: true,
            ..Default::default()
        },
    );
}
