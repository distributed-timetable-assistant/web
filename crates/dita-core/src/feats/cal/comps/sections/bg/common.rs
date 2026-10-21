use leptos::IntoView;
use leptos::html::ElementChild;
use leptos::prelude::{CustomAttribute, GlobalAttributes};
use leptos::svg::{marker, path};

pub fn right_arrow() -> impl IntoView {
    marker()
        .id("right_arrow")
        .attr("viewBox", "0 0 10 8")
        .attr("refX", 10)
        .attr("refY", 4)
        .attr("markerWidth", 10)
        .attr("markerHeight", 8)
        .child(
            path()
                .attr("d", "M 0 1 L 10 4 L 0 7 z")
                .attr("fill", "currentColor"),
        )
}

pub fn up_arrow() -> impl IntoView {
    marker()
        .id("up_arrow")
        .attr("viewBox", "0 0 8 10")
        .attr("refX", 4)
        .attr("refY", 0)
        .attr("markerWidth", 8)
        .attr("markerHeight", 10)
        .child(
            path()
                .attr("d", "M 1 10 L 4 0 L 7 10 z")
                .attr("fill", "currentColor"),
        )
}