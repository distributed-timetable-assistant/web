use crate::feats::cal::comps::sections::common::graph::{StoreCell, Time};
use leptos::html::{ElementChild, div};
use leptos::prelude::{ClassAttribute, CustomAttribute, Get, Memo, Signal};
use leptos::svg::{foreignObject, svg};
use leptos::{IntoView, component};
use leptos_ui::clx::tw_merge;
use models::cal::time_cells::basic::{BasicTimeCell, BasicTimeCellStoreFields};
use models::cal::time_cells::common::{Rule, Status};

#[component]
pub fn BasicTable<S, T>(
    time_cell: S,
    #[prop(optional, into)] class: String,
    width: Signal<u32>,
    height: Signal<u32>,
) -> impl IntoView
where
    S: StoreCell<Cell = BasicTimeCell<T>>,
    T: Time,
{
    let view_box = Memo::new(move |_| format!("0 0 {} {}", width(), height()));
    svg()
        .attr("viewBox", view_box)
        .attr("preserveAspectRatio", "none")
        .class(tw_merge!("stroke-border text-border", class))
        .child(
            foreignObject()
                .attr("width", width)
                .attr("height", height)
                .attr("x", 0)
                .attr("y", 0)
                .child(move || {
                    basic_cell(
                        time_cell.clone().status().get(),
                        time_cell.clone().div_cap().get(),
                        time_cell.clone().rules().get(),
                    )
                }),
        )
}

pub fn basic_cell(status: Status, div_cap: u16, rules: Vec<Rule>) -> impl IntoView {
    let label = if rules.is_empty() {
        "No Rules".to_owned()
    } else {
        format!("{} Rules", rules.len())
    };
    let annotate = match div_cap {
        0 => "∞".to_owned(),
        1 => "1".to_owned(),
        _ => format!("1 / {}", div_cap),
    };
    div()
        .class(tw_merge!(
            r#"relative flex items-center justify-center rounded-sm border-transparent
            w-full h-full border border-border hover:border-primary"#,
            get_theme(&status)
        ))
        .child((
            div().class("text-xs").child(label),
            div()
                .class("absolute bottom-0 right-1 text-[8px]")
                .child(annotate),
        ))
}

fn get_theme(status: &Status) -> &'static str {
    match status {
        Status::Available => "bg-avail text-avail-foreground",
        Status::Unavailable => "bg-unavail text-unavail-foreground",
        Status::Preferred => "bg-pref text-pref-foreground",
        Status::Undesired => "bg-undesired text-undesired-foreground",
    }
}
