use dita_design_system::hooks::header::HeaderState;
use leptos::prelude::*;

#[component]
pub fn TimetablePage() -> impl IntoView {
    let header_ctx = HeaderState::new();
    header_ctx.set_page_title("Timetable".into());

    view! {
        <div class="flex w-full h-full items-center justify-center text-muted-foreground">
            Timetable Page
        </div>
    }
}
