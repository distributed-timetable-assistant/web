use dita_design_system::hooks::header::HeaderMode;
use leptos::prelude::*;

#[component]
pub fn WalletPage() -> impl IntoView {
    let header_ctx = HeaderMode::new();
    header_ctx.set_page_title("Wallet".into());

    view! {
        <div class="flex w-full h-full items-center justify-center text-muted-foreground">
            Wallet Page
        </div>
    }
}
