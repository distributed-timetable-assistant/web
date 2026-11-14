use crate::feats::cal::comps::sections::common::graph::{StoreCell, Time};
use chrono::Weekday;
use leptos::html::{ElementChild, div};
use leptos::prelude::{ClassAttribute, CollectView, CustomAttribute, Memo, Signal};
use leptos::svg::{foreignObject, svg};
use leptos::{IntoView, component};
use leptos_ui::clx::tw_merge;
use models::cal::time_cells::weekly::WeeklyTimeCell;

#[component]
pub fn WeeklyTable<S, T>(
    time_cell: S,
    #[prop(optional, into)] class: String,
    width: Signal<u32>,
    height: Signal<u32>,
) -> impl IntoView
where
    S: StoreCell<Cell = WeeklyTimeCell<T>>,
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
                .child(get_none()),
        )
}

fn get_none() -> impl IntoView {
    div()
        .class("flex gap-1 w-full h-full")
        .child(get_weekday_div())
}

fn get_weekday_div() -> impl IntoView {
    get_all_weekdays()
        .iter()
        .map(|d| none_cell())
        .collect_view()
}

fn get_all_weekdays() -> [Weekday; 7] {
    [
        Weekday::Mon,
        Weekday::Tue,
        Weekday::Wed,
        Weekday::Thu,
        Weekday::Fri,
        Weekday::Sat,
        Weekday::Sun,
    ]
}

pub fn none_cell(/* week_day: &Weekday, time_cells: &Vec<WeeklyTimeCell<GeneralTime>> */)
 -> impl IntoView {
    div().class("h-full bg-none flex-1 border border-border/0 hover:border-primary")
    // .child(time_cells.iter())
}
