pub mod graph;

use dita_design_system::components::core::button::{Button, ButtonProps, ButtonSize, ButtonVariant};
use dita_design_system::components::core::collapsible::{CollapsibleTrigger, CollapsibleTriggerProps};
use icons::{ChevronRight, ChevronRightProps, EllipsisVertical, EllipsisVerticalProps};
use leptos::IntoView;
use leptos::children::ToChildren;
use leptos::html::{ElementChild, div};
use leptos::prelude::ClassAttribute;

pub fn header(title: String) -> impl IntoView {
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
            title,
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