mod app;
mod routes;
mod state;
mod section_sidenav;
mod pages;

use leptos::prelude::*;
use crate::app::App;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <App />
        }
    })
}
