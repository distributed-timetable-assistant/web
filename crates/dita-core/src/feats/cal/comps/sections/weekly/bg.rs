use crate::feats::cal::comps::sections::common::{right_arrow, up_arrow};
use leptos::prelude::*;
use leptos::svg::{defs, g, line, svg};
use leptos_ui::clx::tw_merge;

#[component]
pub fn WeeklyBg(
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
                defs().child((right_arrow(), up_arrow())),
                line()
                    .attr("x1", 100)
                    .attr("y1", line_y)
                    .attr("x2", 100)
                    .attr("y2", 15)
                    .attr("marker-end", "url(#up_arrow)"),
                line()
                    .attr("x1", 0)
                    .attr("y1", line_y)
                    .attr("x2", width)
                    .attr("y2", line_y)
                    .attr("marker-end", "url(#right_arrow)"),
            )),
        )
}
