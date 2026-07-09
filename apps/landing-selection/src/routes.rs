use crate::pages::home::HomePage;
use dita_core::common::pages::not_found::NotFound;
use dita_core::common::pages::wallet::WalletPage;
use leptos::attr::AttributeValue;
use leptos::prelude::*;
use leptos_router::components::{Redirect, Route, Routes};
use leptos_router::path;
use dita_design_system::hooks::header::{Crumb, HeaderMode};

#[component]
pub fn AppRoutes() -> impl IntoView {
    let header_ctx = HeaderMode::new();
    let crumbs= vec![
        Crumb {
            title: "Dita".into(),
            path: "/".into(),
        },
        Crumb {
            title: "Landing Selection".into(),
            path: "/".into(),
        },
    ];
    header_ctx.set_crumbs(crumbs);
    
    view! {
        <Routes fallback=NotFound>
            <Route path=path!("/apps") view=HomePage />
            <Route path=path!("/wallet") view=WalletPage />

            <Route path=path!("/") view=|| view! { <Redirect path="/apps"/> } />
        </Routes>
    }
}
