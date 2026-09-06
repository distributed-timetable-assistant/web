use leptos::prelude::*;

use crate::hooks::auth::AuthState;

/// Dedicated page displayed when login is canceled.
/// Explains that login was canceled and provides an explicit sign-in action without automatic redirect loops.
#[component]
pub fn AuthCancelledPage() -> impl IntoView {
    let auth = AuthState::new();
    let on_retry = move |_| auth.login();

    view! {
        <div class="flex min-h-screen w-full items-center justify-center bg-background p-4 text-foreground">
            <div class="w-full max-w-md rounded-xl border border-border bg-card p-6 shadow-lg text-center space-y-4">
                <div class="mx-auto flex size-12 items-center justify-center rounded-full bg-amber-500/10 text-amber-500">
                    <svg xmlns="http://www.w3.org/2000/svg" class="size-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
                    </svg>
                </div>
                <h1 class="text-xl font-semibold tracking-tight">"Login Cancelled"</h1>
                <p class="text-sm text-muted-foreground">
                    "You canceled the sign-in process. Please sign in to access your account."
                </p>
                <div class="pt-2">
                    <button
                        id="auth-retry-login-btn"
                        on:click=on_retry
                        class="inline-flex h-9 w-full items-center justify-center rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground shadow-xs transition-colors hover:bg-primary/90 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring cursor-pointer"
                    >
                        "Sign In"
                    </button>
                </div>
            </div>
        </div>
    }
}
