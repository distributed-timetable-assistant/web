use crate::feats::cal::comps::sections::common::graph::PointLabel;
use crate::feats::cal::comps::sections::tables::common;
use leptos::prelude::*;
use leptos::svg::{foreignObject, svg};
use leptos_ui::tw_merge::tw_merge;
use models::cal::time_cells::basic::{BasicTimeCell, BasicTimeCellStoreFields};
use reactive_stores::StoreField;

#[component]
pub fn BasicTable<T, S>(
    time_cell: S,
    #[prop(optional, into)] class: String,
    width: Signal<u32>,
    height: Signal<u32>,
) -> (impl IntoView, Signal<Vec<PointLabel<T>>>)
where
    S: StoreField<Value = BasicTimeCell<T>> + Track + Copy,
    T: Send + Sync + 'static,
{
    let view_box = Memo::new(move |_| format!("0 0 {} {}", width(), height()));
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
                    .child(move || {
                        common::basic_cell(
                            time_cell.status().get(),
                            time_cell.div_cap().get(),
                            time_cell.rules().get(),
                        )
                    }),
            ),
        vec![
            PointLabel {
                point: (0.0, 0.5),
                time: time_cell.start(),
            },
            PointLabel {
                point: (1.0, 0.5),
                time: time_cell.end(),
            },
        ],
    )
}
