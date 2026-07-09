use dita_design_system::hooks::header::{Crumb, HeaderMode};
use leptos::prelude::*;

#[component]
pub fn TimetablePage() -> impl IntoView {
    let header_ctx = HeaderMode::new();
    header_ctx.set_page_title("Timetable".into());

    view! {
        <div class="flex w-full h-full items-center justify-center text-muted-foreground">
            Timetable Page
        </div>
    }
}
