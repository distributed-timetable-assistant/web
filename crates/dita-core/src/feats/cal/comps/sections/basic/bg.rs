use crate::feats::cal::comps::sections;
use leptos::html::ElementChild;
use leptos::prelude::{ClassAttribute, CustomAttribute, Memo, Signal};
use leptos::svg::{defs, g, line, svg};
use leptos::{IntoView, component};
use leptos_ui::clx::tw_merge;

#[component]
pub fn BasicBg(
    #[prop(optional, into)] class: String,
    width: Signal<u32>,
    height: Signal<u32>,
) -> impl IntoView {
    let view_box = Memo::new(move |_| format!("0 0 {} {}", width(), height()));
    let line_y = Memo::new(move |_| height() - 15);
    svg()
        .attr("viewBox", view_box)
        .attr("preserveAspectRatio", "none")
        .class(tw_merge!("stroke-border text-border", class))
        .child(
            g().class("pointer-events-none").child((
                defs().child(sections::common::right_arrow()),
                line()
                    .attr("x1", 0)
                    .attr("y1", line_y)
                    .attr("x2", width)
                    .attr("y2", line_y)
                    .attr("marker-end", "url(#right_arrow)"),
            )),
        )
}