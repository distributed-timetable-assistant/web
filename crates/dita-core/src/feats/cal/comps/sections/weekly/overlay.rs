use leptos::{component, IntoView};
use leptos::prelude::Signal;

#[component]
pub fn WeeklyOverlay(
    #[prop(optional, into)] class: String,
    width: Signal<u32>,
    height: Signal<u32>,
    table_x_offset: u32,
    table_y_offset: u32,
    table_width: Signal<u32>,
    table_height: Signal<u32>,
) -> impl IntoView {
    
}