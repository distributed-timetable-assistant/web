use crate::feats::cal::comps::sections::{basic, common, weekly};
use dita_design_system::components::core::collapsible::{
    Collapsible, CollapsibleContent, CollapsibleContentProps, CollapsibleProps,
};
use leptos::html::div;
use leptos::prelude::*;
use leptos_ui::tw_merge::tw_merge;
use models::cal::time_cells::common::Cell;
use models::cal::{GeneralTimeCell, GeneralTimeCellStoreFields};
use reactive_stores::StoreField;

#[component]
pub fn Avail<C>(avail: C) -> impl IntoView
where
    C: StoreField<Value = GeneralTimeCell>
        + Get<Value = GeneralTimeCell>
        + Track
        + Copy
        + Send
        + Sync
        + 'static,
{
    let open = RwSignal::new(true);
    div()
        .class(tw_merge!("w-full p-4"))
        .child(match avail.get() {
            // GeneralTimeCell::Yearly(_) => {}
            // GeneralTimeCell::Monthly(_) => {}
            GeneralTimeCell::Weekly(_) => weekly::general_weekly_collapse(avail.weekly_0().unwrap()),
            // GeneralTimeCell::Daily(_) => {}
            GeneralTimeCell::Basic(_) => basic::general_basic_collapse(avail.basic_0().unwrap()),
            // GeneralTimeCell::Holiday(_) => {}
            _ => Collapsible(
                CollapsibleProps::builder()
                    .open(open)
                    .children(ToChildren::to_children(move || {
                        (
                            common::header(avail.get().id().to_string()),
                            CollapsibleContent(
                                CollapsibleContentProps::builder()
                                    .children(ToChildren::to_children(move || {
                                        div().class("px-2").child(format!("{:?}", avail.get()))
                                    }))
                                    .build(),
                            ),
                        )
                    }))
                    .build(),
            )
            .into_any(),
        })
}
