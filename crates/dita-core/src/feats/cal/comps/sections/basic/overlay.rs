use crate::feats::cal::comps::sections::common::graph::{PointLabel, Time};
use leptos::html::{ElementChild, div};
use leptos::prelude::*;
use leptos::svg::{foreignObject, g, line, svg};
use leptos::{IntoView, component};
use leptos_ui::clx::tw_merge;

#[component]
pub fn BasicOverlay<T: Time>(
    #[prop(optional, into)] class: String,
    width: Signal<u32>,
    height: Signal<u32>,
    table_x_offset: u32,
    table_y_offset: u32,
    table_width: Signal<u32>,
    table_height: Signal<u32>,
    points: Vec<PointLabel<T>>,
) -> impl IntoView {
    let view_box = Memo::new(move |_| format!("0 0 {} {}", width(), height()));
    let lines = points
        .iter()
        .map(|pl| {
            line()
                .attr(
                    "x1",
                    table_x_offset as f32 + pl.loc.x * table_width() as f32,
                )
                .attr(
                    "y1",
                    table_y_offset as f32 + pl.loc.y * table_height() as f32,
                )
                .attr(
                    "x2",
                    table_x_offset as f32 + pl.loc.x * table_width() as f32,
                )
                .attr("y2", height() - 15)
        })
        .collect_view();
    let footer = foreignObject()
        .attr("width", move || width())
        .attr("height", 10)
        .attr("x", 0)
        .attr("y", move || height() - 10)
        .child(
            div()
                .class("flex justify-between items-center w-full text-muted-foreground")
                .child(
                    points
                        .iter()
                        .map(|pl| div().child(pl.time.get_label()))
                        .collect_view(),
                ),
        );
    svg()
        .attr("viewBox", view_box)
        .attr("preserveAspectRatio", "none")
        .class(tw_merge!(
            "stroke-muted-foreground/50 text-border pointer-events-none text-xs",
            class
        ))
        .attr("stroke-dasharray", "2 4")
        .child(g().child((lines, footer)))
}
