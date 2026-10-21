use leptos::prelude::*;

use crate::hooks::auth::{AuthState, AuthStatus};

/// Dedicated page displayed when authentication fails due to OIDC errors or state validation errors.
/// Displays error details and provides an explicit retry button without automatic redirect loops.
#[component]
pub fn AuthErrorPage() -> impl IntoView {
    let auth = AuthState::new();
    let status = auth.status();
    let on_retry = move |_| auth.login();

    let error_message = move || match status.get() {
        AuthStatus::Error(err) => err.to_string(),
        _ => "An unknown authentication error occurred.".to_string(),
    };

    view! {
        <div class="flex min-h-screen w-full items-center justify-center bg-background p-4 text-foreground">
            <div class="w-full max-w-md rounded-xl border border-border bg-card p-6 shadow-lg text-center space-y-4">
                <div class="mx-auto flex size-12 items-center justify-center rounded-full bg-destructive/10 text-destructive">
                    <svg xmlns="http://www.w3.org/2000/svg" class="size-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                    </svg>
                </div>
                <h1 class="text-xl font-semibold tracking-tight">"Authentication Error"</h1>
                <p class="text-sm text-muted-foreground">
                    "An error occurred while attempting to sign in. Please try again."
                </p>
                <div class="text-left bg-muted/50 rounded-lg p-3 text-xs font-mono text-muted-foreground overflow-x-auto">
                    {error_message}
                </div>
                <div class="pt-2">
                    <button
                        id="auth-error-retry-btn"
                        on:click=on_retry
                        class="inline-flex h-9 w-full items-center justify-center rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground shadow-xs transition-colors hover:bg-primary/90 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring cursor-pointer"
                    >
                        "Try Again"
                    </button>
                </div>
            </div>
        </div>
    }
}
