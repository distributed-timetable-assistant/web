pub mod graph;

use crate::feats::cal::comps::sections::common::graph::{StoreCell, Time};
use chrono::NaiveTime;
use dita_design_system::components::core::button::{
    Button, ButtonProps, ButtonSize, ButtonVariant,
};
use dita_design_system::components::core::collapsible::{
    CollapsibleTrigger, CollapsibleTriggerProps,
};
use icons::{ChevronRight, ChevronRightProps, EllipsisVertical, EllipsisVerticalProps};
use leptos::children::{Children, ToChildren};
use leptos::html::{ElementChild, div};
use leptos::prelude::*;
use leptos::prelude::{ClassAttribute, CustomAttribute, GlobalAttributes};
use leptos::svg::{marker, path};
use models::cal::time_cells::basic::BasicTimeCell;
use models::cal::time_cells::monthly::MonthTime;
use models::cal::time_cells::weekly::{WeekTime, WeeklyTimeCell};
use models::cal::time_cells::yearly::YearTime;
use models::cal::{Cal, GeneralTime, GeneralTimeCell};
use reactive_stores::{AtKeyed, Store, Subfield};
use uuid::Uuid;

impl Time for GeneralTime {
    fn get_label(&self) -> String {
        self.time().to_string()
    }
}
impl Time for YearTime {
    fn get_label(&self) -> String {
        todo!()
    }
}
impl Time for MonthTime {
    fn get_label(&self) -> String {
        todo!()
    }
}
impl Time for WeekTime {
    fn get_label(&self) -> String {
        self.time.to_string()
    }
}
impl Time for NaiveTime {
    fn get_label(&self) -> String {
        self.to_string()
    }
}
impl StoreCell
    for Subfield<
        AtKeyed<Store<Cal>, Cal, Uuid, Vec<GeneralTimeCell>>,
        GeneralTimeCell,
        WeeklyTimeCell<GeneralTime>,
    >
{
    type Cell = WeeklyTimeCell<GeneralTime>;
}

impl StoreCell
    for Subfield<
        AtKeyed<Store<Cal>, Cal, Uuid, Vec<GeneralTimeCell>>,
        GeneralTimeCell,
        BasicTimeCell<GeneralTime>,
    >
{
    type Cell = BasicTimeCell<GeneralTime>;
}

pub fn header(children: Children) -> impl IntoView {
    div()
        .class("w-full border border-border rounded-sm mb-3 text-sm flex gap-2 items-center px-2 py-1 bg-black/20")
        .child((
            CollapsibleTrigger(CollapsibleTriggerProps::builder()
                .class("flex text-sm transition-colors group")
                .children(
                    ToChildren::to_children(move || ChevronRight(ChevronRightProps::builder()
                        .class("transition-transform duration-200 size-4 group-data-[state=open]:rotate-90")
                        .build()))
                )
                .build()),
            children(),
            Button(ButtonProps::builder()
                .variant(ButtonVariant::Ghost)
                .size(ButtonSize::IconXs)
                .class("ml-auto")
                .children(ToChildren::to_children(move || {
                    EllipsisVertical(EllipsisVerticalProps::builder().build())
                }))
                .build()),
        ))
}

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
