use dita_design_system::hooks::header::HeaderState;
use leptos::prelude::*;

#[component]
pub fn FacilitiesPage() -> impl IntoView {
    let header_ctx = HeaderState::new();
    header_ctx.set_page_title("Facilities".into());

    view! {
        <div class="flex w-full h-full items-center justify-center text-muted-foreground">
            Facilities Page
        </div>
    }
}
