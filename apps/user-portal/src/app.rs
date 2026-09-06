use crate::pages::calendar::CalendarPage;
use crate::pages::capabilities::CapabilitiesPage;
use crate::pages::courses::CoursesPage;
use crate::pages::qualifications::QualificationsPage;
use crate::pages::resources::ResourcesPage;
use crate::pages::subjects::SubjectsPage;
use crate::pages::timetable::TimetablePage;
use crate::section_sidenav::get_section_data;
use dita_core::common::app::{AppBase, AppBaseProps};
use dita_core::common::pages::wallet::WalletPage;
use dita_core::routes;
use dita_design_system::hooks::header::Crumb;
use dita_state::app_state::AppState;
use leptos::prelude::*;
use leptos_router::components::Redirect;

const PUBLIC_URL: Option<&'static str> = option_env!("TRUNK_BUILD_PUBLIC_URL");

#[component]
pub fn App() -> impl IntoView {
    let crumbs = vec![
        Crumb {
            title: "Dita".into(),
            path: "/".into(),
        },
        Crumb {
            title: "User".into(),
            path: "/user".into(),
        },
    ];
    AppBase(
        AppBaseProps::builder()
            .state_builder(AppState::builder())
            .base(PUBLIC_URL.unwrap_or_default())
            .base_crumbs(crumbs)
            .section_data(get_section_data())
            .routes(routes![
                "/calendar" => CalendarPage,
                "/capabilities" => CapabilitiesPage,
                "/subjects" => SubjectsPage,
                "/resources" => ResourcesPage,
                "/courses" => CoursesPage,
                "/qualifications" => QualificationsPage,
                "/timetable" => TimetablePage,
                "/wallet" => WalletPage,
                "/" => || view! { <Redirect path="/calendar"/> },
            ])
            .build(),
    )
}
