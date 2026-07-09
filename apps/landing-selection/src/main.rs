use crate::app::App;
use leptos::prelude::*;

mod app;
mod routes;
mod state;
mod section_sidenav;
mod pages;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <App />
        }
    })
}
