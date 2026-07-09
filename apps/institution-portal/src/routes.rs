use crate::pages::actors::ActorsPage;
use crate::pages::courses::CoursesPage;
use crate::pages::facilities::FacilitiesPage;
use crate::pages::groups::GroupsPage;
use crate::pages::learners::LearnersPage;
use crate::pages::packets::PacketsPage;
use crate::pages::resources::ResourcesPage;
use crate::pages::subjects::SubjectsPage;
use crate::pages::timetable::TimetablePage;
use crate::pages::units::UnitsPage;
use dita_core::common::pages::not_found::NotFound;
use dita_core::common::pages::wallet::WalletPage;
use dita_design_system::hooks::header::{Crumb, HeaderMode};
use leptos::prelude::*;
use leptos_router::components::{Redirect, Route, Routes};
use leptos_router::path;
use crate::pages::calendar::CalendarPage;

#[component]
pub fn AppRoutes() -> impl IntoView {
    let header_ctx = HeaderMode::new();
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
    header_ctx.set_crumbs(crumbs);

    view! {
        <Routes fallback=NotFound>
            <Route path=path!("/calendar") view=CalendarPage />
            <Route path=path!("/units") view=UnitsPage />
            <Route path=path!("/facilities") view=FacilitiesPage />
            <Route path=path!("/groups") view=GroupsPage />
            <Route path=path!("/learners") view=LearnersPage />
            <Route path=path!("/subjects") view=SubjectsPage />
            <Route path=path!("/resources") view=ResourcesPage />
            <Route path=path!("/actors") view=ActorsPage />
            <Route path=path!("/courses") view=CoursesPage />
            <Route path=path!("/packets") view=PacketsPage />
            <Route path=path!("/timetable") view=TimetablePage />
            <Route path=path!("/wallet") view=WalletPage />

            <Route path=path!("/") view=|| view! { <Redirect path="/calendar"/> } />
        </Routes>
    }
}
