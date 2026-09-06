use crate::hooks::auth::{AuthState, AuthStatus};
use crate::pages::loading::AuthLoadingPage;
use crate::state_mgmt::store_target_destination;
/// `RequireAuth` – a reusable route guard for DiTA applications.
///
/// Wrap any protected route content (such as `AppShell` and `Outlet`) in `<RequireAuth>` to enforce
/// authentication. Header, sidebar, and application shell are never rendered while authentication is
/// unresolved, canceled, or failed.
///
/// | State          | Behavior                                       |
/// |----------------|------------------------------------------------|
/// | Loading        | Render in-place loading UI                     |
/// | Authenticated  | Render children (AppShell)                     |
/// | Unauthenticated| Trigger login redirect (once)                  |
/// | LoginCancelled | Navigate to /canceled                          |
/// | StateError     | Navigate to /error                             |
/// | Error          | Navigate to /error                             |
use leptos::prelude::*;
use leptos_router::NavigateOptions;
use leptos_router::hooks::{use_location, use_navigate};

#[component]
pub fn RequireAuth(children: ChildrenFn) -> impl IntoView {
    let auth = AuthState::new();
    let status = auth.status();
    let navigate = use_navigate();

    // Memoize unauthenticated status to trigger login once
    let is_unauthenticated =
        Memo::new(move |_| matches!(status.get(), AuthStatus::Unauthenticated));

    Effect::new(move |_| {
        if is_unauthenticated.get() {
            let location = use_location();
            let path = location.pathname.get_untracked();
            let search = location.search.get_untracked();
            let hash = location.hash.get_untracked();
            let target = format!(
                "{path}{}{hash}",
                (!search.is_empty())
                    .then(|| format!("?{search}"))
                    .unwrap_or_default(),
            );
            store_target_destination(&target);
            auth.login();
        }
    });

    Effect::new(move |_| match status.get() {
        AuthStatus::LoginCanceled => {
            navigate(
                "canceled",
                NavigateOptions {
                    resolve: true,
                    replace: true,
                    ..Default::default()
                },
            );
        }
        AuthStatus::Error(_) | AuthStatus::StateError(_) => {
            navigate(
                "error",
                NavigateOptions {
                    resolve: true,
                    replace: true,
                    ..Default::default()
                },
            );
        }
        _ => {}
    });

    view! {
        {move || match status.get() {
            AuthStatus::Authenticated => children().into_any(),
            _ => view! { <AuthLoadingPage /> }.into_any(),
        }}
    }
}
