use crate::pages::actors::ActorsPage;
use crate::pages::calendar::CalendarPage;
use crate::pages::courses::CoursesPage;
use crate::pages::facilities::FacilitiesPage;
use crate::pages::groups::GroupsPage;
use crate::pages::learners::LearnersPage;
use crate::pages::packets::PacketsPage;
use crate::pages::resources::ResourcesPage;
use crate::pages::subjects::SubjectsPage;
use crate::pages::timetable::TimetablePage;
use crate::pages::units::UnitsPage;
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
            title: "Institution".into(),
            path: "/institution".into(),
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
                "/units" => UnitsPage,
                "/facilities" => FacilitiesPage,
                "/groups" => GroupsPage,
                "/learners" => LearnersPage,
                "/subjects" => SubjectsPage,
                "/resources" => ResourcesPage,
                "/actors" => ActorsPage,
                "/courses" => CoursesPage,
                "/packets" => PacketsPage,
                "/timetable" => TimetablePage,
                "/wallet" => WalletPage,
                "/" => || view! { <Redirect path="/calendar"/> },
            ])
            .build(),
    )
}
