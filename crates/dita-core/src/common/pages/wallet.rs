use leptos::prelude::*;
use crate::common::hooks::header::HeaderState;

#[component]
pub fn WalletPage() -> impl IntoView {
    let header_ctx = HeaderState::new();
    header_ctx.set_page_title("Wallet".into());

    view! {
        <div class="flex w-full h-full items-center justify-center text-muted-foreground">
            Wallet Page
        </div>
    }
}
