use crate::feats::cal::comps::sections::weekly::WeeklyPointLabel;
use chrono::Weekday;
use leptos::html::div;
use leptos::prelude::*;
use leptos::svg::{foreignObject, svg};
use leptos::{IntoView, component};
use leptos_ui::clx::tw_merge;
use models::cal::GeneralTime;
use models::cal::time_cells::common::TimeCellStoreFields;
use models::cal::time_cells::weekly::WeeklyTimeCell;
use reactive_stores::StoreField;
use crate::feats::cal::comps::sections::tables::common::none_cell;

#[component]
pub fn GeneralWeeklyTable<C>(
    time_cell: C,
    #[prop(optional, into)] class: String,
    width: Signal<u32>,
    #[prop(into)] height: u32,
) -> (impl IntoView, Vec<WeeklyPointLabel>)
where
    C: StoreField<Value = WeeklyTimeCell<GeneralTime>> + Track + Copy + Send + Sync + 'static,
{
    let view_box = Memo::new(move |_| format!("0 0 {} {}", width(), height));
    (
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
            ),
        vec![],
    )
}

fn get_none() -> impl IntoView {
    div().class("flex gap-1 w-full h-full").child(get_weekday_div())
}

fn get_weekday_div() -> impl IntoView {
    get_all_weekdays()
        .iter()
        .map(|d| {
            none_cell()
        })
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
