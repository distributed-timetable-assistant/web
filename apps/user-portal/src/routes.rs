use crate::pages::calendar::CalendarPage;
use dita_core::common::pages::not_found::NotFound;
use dita_core::common::pages::wallet::WalletPage;
use dita_design_system::hooks::header::{Crumb, HeaderMode};
use leptos::prelude::*;
use leptos_router::components::{Redirect, Route, Routes};
use leptos_router::path;
use crate::pages::capabilities::CapabilitiesPage;
use crate::pages::courses::CoursesPage;
use crate::pages::qualifications::QualificationsPage;
use crate::pages::resources::ResourcesPage;
use crate::pages::subjects::SubjectsPage;
use crate::pages::timetable::TimetablePage;

#[component]
pub fn AppRoutes() -> impl IntoView {
    let header_ctx = HeaderMode::new();
    let crumbs= vec![
        Crumb {
            title: "Dita".into(),
            path: "/".into(),
        },
        Crumb {
            title: "User".into(),
            path: "/user".into(),
        },
    ];
    header_ctx.set_crumbs(crumbs);

    view! {
        <Routes fallback=NotFound>
            <Route path=path!("/calendar") view=CalendarPage />
            <Route path=path!("/capabilities") view=CapabilitiesPage />
            <Route path=path!("/subjects") view=SubjectsPage />
            <Route path=path!("/resources") view=ResourcesPage />
            <Route path=path!("/courses") view=CoursesPage />
            <Route path=path!("/qualifications") view=QualificationsPage />
            <Route path=path!("/timetable") view=TimetablePage />
            <Route path=path!("/wallet") view=WalletPage />

            <Route path=path!("/") view=|| view! { <Redirect path="/calendar"/> } />
        </Routes>
    }
}
