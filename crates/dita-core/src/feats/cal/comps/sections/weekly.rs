use chrono::{NaiveTime, Weekday};
use glam::Vec2;
use crate::feats::cal::comps::sections::bg::weekly::{GeneralWeeklyBg, GeneralWeeklyBgProps};
use crate::feats::cal::comps::sections::common::header;
use dita_design_system::components::core::collapsible::{
    Collapsible, CollapsibleContent, CollapsibleContentProps, CollapsibleProps,
};
use leptos::html::{Div, div};
use leptos::prelude::*;
use leptos_use::use_resize_observer;
use models::cal::GeneralTime;
use models::cal::time_cells::weekly::WeeklyTimeCell;
use reactive_stores::StoreField;
use crate::feats::cal::comps::sections::tables::weekly::{GeneralWeeklyTable, GeneralWeeklyTableProps};

pub fn general_weekly_collapse<C>(time_cell: C) -> AnyView
where
    C: StoreField<Value = WeeklyTimeCell<GeneralTime>> + Get + Track + Copy + Send + Sync + 'static,
{
    let open = RwSignal::new(true);
    Collapsible(
        CollapsibleProps::builder()
            .open(open)
            .children(ToChildren::to_children(move || {
                (
                    header("Weekly".to_owned()),
                    CollapsibleContent(
                        CollapsibleContentProps::builder()
                            .children(ToChildren::to_children(move || {
                                general_basic_table(time_cell)
                            }))
                            .build(),
                    ),
                )
            }))
            .build(),
    )
    .into_any()
}

#[derive(Clone)]
pub struct WeeklyPointLabel {
    pub point: Vec2,
    pub hour: NaiveTime,
    pub w_day: Weekday,
}

fn general_basic_table<C>(time_cell: C) -> AnyView
where
    C: StoreField<Value = WeeklyTimeCell<GeneralTime>> + Get + Track + Copy + Send + Sync + 'static,
{
    let height: u32 = 500;
    let parent_ref = NodeRef::<Div>::new();
    let (parent_width, set_parent_width) = signal(1000);
    use_resize_observer(parent_ref, move |entries, _| {
        let rect = entries[0].content_rect();
        set_parent_width.set(rect.width() as u32);
    });

    let table_y_margin: u32 = 15;
    let table_height: u32 = height - 2 * table_y_margin;
    let table_x_margin: u32 = 85;
    let table_width = Memo::new(move |_| parent_width.get() - 2 * table_x_margin);
    let (table, points) = get_table(time_cell, table_width.into(), table_height, table_x_margin, table_y_margin);

    div()
        .node_ref(parent_ref)
        .class("grid")
        .child((
            get_bg(parent_width.into(), height),
            table,
            // get_overlay(
            //     parent_width.into(),
            //     height,
            //     table_x_margin,
            //     0,
            //     table_width.into(),
            //     table_height,
            //     points,
            // ),
        ))
        .into_any()
}

fn get_bg(width: Signal<u32>, height: u32) -> impl IntoView {
    GeneralWeeklyBg(
        GeneralWeeklyBgProps::builder()
            .class("col-start-1 row-start-1")
            .width(width)
            .height(height)
            .build(),
    )
}

fn get_table<C>(
    time_cell: C,
    width: Signal<u32>,
    height: u32,
    x_margin: u32,
    y_margin: u32,
) -> (impl IntoView, Vec<WeeklyPointLabel>)
where
    C: StoreField<Value = WeeklyTimeCell<GeneralTime>> + Get + Track + Copy + Send + Sync + 'static,
{
    GeneralWeeklyTable(
        GeneralWeeklyTableProps::builder()
            .time_cell(time_cell)
            .class(format!("col-start-1 row-start-1 mx-[{}px] my-[{}px]", x_margin, y_margin))
            .width(width)
            .height(height)
            .build(),
    )
}
