use leptos::prelude::*;

/// Presentation-only loading indicator component shown during authentication operations.
/// Does not perform any navigation or side effects.
#[component]
pub fn AuthLoadingPage() -> impl IntoView {
    view! {
        <div class="flex h-screen w-full items-center justify-center bg-background text-foreground" aria-live="polite" aria-busy="true">
            <div class="flex flex-col items-center gap-4 p-8">
                <div class="size-8 animate-spin rounded-full border-4 border-primary border-t-transparent"></div>
                <p class="text-sm text-muted-foreground font-medium">"Authenticating…"</p>
            </div>
        </div>
    }
}
