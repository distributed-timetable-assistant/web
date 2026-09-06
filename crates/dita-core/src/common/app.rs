use crate::common::components::section_sidenav::{SectionData, create_section_sidenav};
use crate::common::components::sidebar::Sidebar;
use crate::common::pages::not_found::NotFound;
use dita_auth::{AuthCallbackPage, AuthCancelledPage, AuthErrorPage, AuthState, RequireAuth};
use dita_design_system::components::header::layout::Header;
use dita_design_system::hooks::header::{Crumb, HeaderState};
use dita_design_system::hooks::sidenav::SidenavState;
use dita_design_system::hooks::theme::ThemeState;
use dita_state::app_state::AppStateBuilder;
use leptos::prelude::*;
use leptos_router::{components::*, *};

#[component]
pub fn AppBase<C>(
    state_builder: AppStateBuilder,
    base: &'static str,
    routes: RouteChildren<C>,
    base_crumbs: Vec<Crumb>,
    section_data: SectionData,
) -> impl IntoView
where
    C: Clone + Send + 'static + MatchNestedRoutes,
{
    build_state(state_builder);
    let header_ctx = HeaderState::new();
    header_ctx.set_crumbs(base_crumbs);
    Router(
        RouterProps::builder()
            .base(base)
            .children(ToChildren::to_children(|| {
                Routes(
                    RoutesProps::builder()
                        .fallback(NotFound)
                        .children(RouteChildren::to_children(||
                            (
                                Route(
                                    RouteProps::builder()
                                        .path(StaticSegment("/callback"))
                                        .view(|| view! { <AuthCallbackPage destination="../" /> })
                                        .build(),
                                ),
                                Route(
                                    RouteProps::builder()
                                        .path(StaticSegment("/error"))
                                        .view(AuthErrorPage)
                                        .build(),
                                ),
                                Route(
                                    RouteProps::builder()
                                        .path(StaticSegment("/canceled"))
                                        .view(AuthCancelledPage)
                                        .build(),
                                ),
                                ParentRoute(
                                    ParentRouteProps::builder()
                                        .path(())
                                        .view(move || view! { <AppShell section_data=section_data.clone() /> })
                                        .children(routes)
                                        .build(),
                                ),
                            )
                        ))
                        .build(),
                )
            }))
            .build(),
    )
}

fn build_state(state_builder: AppStateBuilder) {
    state_builder
        .with::<ThemeState>()
        .with::<SidenavState>()
        .with::<HeaderState>()
        .with::<AuthState>()
        .build();
}

#[component]
fn AppShell(section_data: SectionData) -> impl IntoView {
    view! {
        <RequireAuth>
            <div class="flex h-full w-full" style="--sidenav-width:12rem;--sidenav-width-icon:3rem">
                <Sidebar/>
                {create_section_sidenav(section_data.clone())}
                <div class="flex flex-1 flex-col">
                    <Header />
                    <div class="min-h-0 flex-1 overflow-y-auto">
                        <Outlet/>
                    </div>
                </div>
            </div>
        </RequireAuth>
    }
}

#[macro_export]
macro_rules! route {
    ($path:literal => $view:expr) => {
        leptos_router::components::Route(
            leptos_router::components::RouteProps::builder()
                .path(leptos_router::StaticSegment($path))
                .view($view)
                .build(),
        )
    };

    (() => $view:expr) => {
        leptos_router::components::Route(
            leptos_router::components::RouteProps::builder()
                .path(())
                .view($view)
                .build(),
        )
    };
}

#[macro_export]
macro_rules! routes {
    ($($path:tt => $view:expr),+ $(,)?) => {
        leptos_router::components::RouteChildren::to_children(|| {
            (
                $(
                    $crate::route!($path => $view)
                ),+
            )
        })
    };
}
