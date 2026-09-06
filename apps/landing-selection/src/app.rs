use crate::pages::home::HomePage;
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
            title: "Landing Selection".into(),
            path: "/".into(),
        },
    ];
    AppBase(
        AppBaseProps::builder()
            .state_builder(AppState::builder())
            .base(PUBLIC_URL.unwrap_or_default())
            .base_crumbs(crumbs)
            .section_data(get_section_data())
            .routes(routes![
                "/apps" => HomePage,
                "/wallet" => WalletPage,
                "/" => || view! { <Redirect path="/apps"/> },
            ])
            .build(),
    )
}
