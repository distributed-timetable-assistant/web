use crate::app::App;
use leptos::prelude::*;

mod app;
mod pages;
mod section_sidenav;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <App />
        }
    })
}
