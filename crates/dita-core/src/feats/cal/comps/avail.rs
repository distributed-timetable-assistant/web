use crate::feats::cal::comps::sections::common::graph::{CalGraphDraw, StoreCell, Time};
use crate::feats::cal::comps::sections::common::header;
use dita_design_system::components::core::collapsible::{
    Collapsible, CollapsibleContent, CollapsibleContentProps, CollapsibleProps,
};
use leptos::html::div;
use leptos::prelude::*;
use leptos_ui::tw_merge::tw_merge;
use models::cal::time_cells::common::Cell;
use models::cal::{Cal, GeneralTimeCell};
use reactive_stores::{AtKeyed, Store};
use uuid::Uuid;

#[component]
pub fn Avail(avail: AtKeyed<Store<Cal>, Cal, Uuid, Vec<GeneralTimeCell>>) -> impl IntoView {
    let open = RwSignal::new(true);
    div()
        .class(tw_merge!("w-full p-4"))
        .child(move || match avail.get() {
            // GeneralTimeCell::Yearly(_) => {}
            // GeneralTimeCell::Monthly(_) => {}
            // Weekly(_) => {
            //     let graph = CalGraph {
            //         time_cell: avail.weekly_0().unwrap(),
            //     };
            //     get_collapse("Weekly".into(), graph)
            // }
            // GeneralTimeCell::Daily(_) => {}
            // Basic(_) => {
            //     let graph = CalGraph {
            //         time_cell: avail.basic_0().unwrap(),
            //     };
            //     get_collapse("Basic".into(), graph)
            // }
            // GeneralTimeCell::Holiday(_) => {}
            _ => Collapsible(
                CollapsibleProps::builder()
                    .open(open)
                    .children(ToChildren::to_children(move || {
                        (
                            header(ToChildren::to_children(move || {
                                avail.get().id().to_string()
                            })),
                            CollapsibleContent(
                                CollapsibleContentProps::builder()
                                    .children(ToChildren::to_children(move || {
                                        div()
                                            .class("px-2")
                                            .child(move || format!("{:?}", avail.get()))
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

fn get_collapse<S, C, T>(
    title: String,
    graph: impl CalGraphDraw<S, C, T> + Send + 'static,
) -> AnyView
where
    S: StoreCell<Cell = C>,
    C: Cell + 'static,
    T: Time,
{
    let open = RwSignal::new(true);
    Collapsible(
        CollapsibleProps::builder()
            .open(open)
            .children(ToChildren::to_children(move || {
                (
                    header(ToChildren::to_children(move || title)),
                    CollapsibleContent(
                        CollapsibleContentProps::builder()
                            .children(ToChildren::to_children(move || {
                                graph.draw("w-full h-[70px]")
                            }))
                            .build(),
                    ),
                )
            }))
            .build(),
    )
    .into_any()
}
