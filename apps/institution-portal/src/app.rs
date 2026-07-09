use crate::routes::AppRoutes;
use crate::section_sidenav::SectionSidenav;
use crate::state;
use dita_core::common::components::sidebar::Sidebar;
use dita_design_system::components::header::layout::Header;
use leptos::prelude::*;
use leptos_router::components::Router;

const PUBLIC_URL: Option<&'static str> = option_env!("TRUNK_BUILD_PUBLIC_URL");

#[component]
pub fn App() -> impl IntoView {
    state::init();

    view! {
        <Router base=PUBLIC_URL.unwrap_or_default()>
            <div class="flex h-full w-full" style="--sidenav-width:12rem;--sidenav-width-icon:3rem">
                <Sidebar/>
                <SectionSidenav/>
                <div class="flex flex-1 flex-col">
                    <Header />
                    <div class="min-h-0 flex-1 overflow-y-auto">
                        <AppRoutes/>
                    </div>
                </div>
            </div>
        </Router>
    }
}
