use crate::feats::cal::comps::sections::bg::basic::{BasicBg, BasicBgProps};
use crate::feats::cal::comps::sections::common;
use crate::feats::cal::comps::sections::common::graph::{CalGraph, CalGraphDraw, Offset, PointLabel, Size, TableGeo};
use crate::feats::cal::comps::sections::overlays::basic::{
    GeneralBasicOverlay, GeneralBasicOverlayProps,
};
use crate::feats::cal::comps::sections::tables::basic::{BasicTable, BasicTableProps};
use dita_design_system::components::core::collapsible::{
    Collapsible, CollapsibleContent, CollapsibleContentProps, CollapsibleProps,
};
use glam::Vec2;
use leptos::IntoView;
use leptos::children::ToChildren;
use leptos::html::{Div, ElementChild, div};
use leptos::prelude::*;
use leptos_use::use_resize_observer;
use models::cal::GeneralTime;
use models::cal::time_cells::basic::BasicTimeCell;
use models::cal::time_cells::weekly::WeeklyTimeCell;
use reactive_stores::StoreField;

pub fn general_basic_collapse<C>(time_cell: C) -> AnyView
where
    C: StoreField<Value = BasicTimeCell<GeneralTime>> + Get + Track + Copy + Send + Sync + 'static,
{
    let open = RwSignal::new(true);
    Collapsible(
        CollapsibleProps::builder()
            .open(open)
            .children(ToChildren::to_children(move || {
                (
                    common::header("Basic".to_owned()),
                    CollapsibleContent(
                        CollapsibleContentProps::builder()
                            .children(ToChildren::to_children(move || {
                                general_basic_graph(time_cell)
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
pub struct BasicPointLabel {
    pub point: Vec2,
    pub g_time: String,
}

fn general_basic_graph<C>(time_cell: C) -> impl IntoView
where
    C: StoreField<Value = BasicTimeCell<GeneralTime>> + Get + Track + Copy + Send + Sync + 'static,
{
    let height: u32 = 70;
    let parent_ref = NodeRef::<Div>::new();
    let (parent_width, set_parent_width) = signal(1000);
    use_resize_observer(parent_ref, move |entries, _| {
        let rect = entries[0].content_rect();
        set_parent_width.set(rect.width() as u32);
    });

    let table_height: u32 = 40;
    let table_x_margin: u32 = 30;
    let table_width = Memo::new(move |_| parent_width.get() - 2 * table_x_margin);
    let (table, points) = get_table(time_cell, table_width.into(), table_height, table_x_margin);

    div()
        .node_ref(parent_ref)
        .class("grid")
        .child((
            get_bg(parent_width.into(), height),
            table,
            get_overlay(
                parent_width.into(),
                height,
                table_x_margin,
                0,
                table_width.into(),
                table_height,
                points,
            ),
        ))
}

fn get_bg(width: Signal<u32>, height: u32) -> impl IntoView {
    BasicBg(
        BasicBgProps::builder()
            .class("col-start-1 row-start-1")
            .width(width)
            .height(height.get())
            .build(),
    )
}

fn get_table<C>(
    time_cell: C,
    width: Signal<u32>,
    height: u32,
    x_margin: u32,
) -> (impl IntoView, Signal<Vec<PointLabel<GeneralTime>>>)
where
    C: StoreField<Value = BasicTimeCell<GeneralTime>> + Get + Track + Copy + Send + Sync + 'static,
{
    BasicTable(
        BasicTableProps::builder()
            .time_cell(time_cell)
            .class(format!("col-start-1 row-start-1 mx-[{}px]", x_margin))
            .width(width)
            .height(height.get())
            .build(),
    )
}

fn get_overlay(
    width: Signal<u32>,
    height: u32,
    table_x_offset: u32,
    table_y_offset: u32,
    table_width: Signal<u32>,
    table_height: u32,
    points: Vec<BasicPointLabel>,
) -> impl IntoView {
    GeneralBasicOverlay(
        GeneralBasicOverlayProps::builder()
            .class("col-start-1 row-start-1")
            .width(width)
            .height(height)
            .table_x_offset(table_x_offset)
            .table_y_offset(table_y_offset)
            .table_width(table_width)
            .table_height(table_height)
            .points(points)
            .build(),
    )
}

impl<T, S> CalGraphDraw<T> for CalGraph<S>
where
    S: StoreField<Value = WeeklyTimeCell<T>> + Copy,
    T: Send + Sync + 'static
{
    fn draw_bg(&self, size: Size) -> (impl IntoView, TableGeo) {
        todo!()
    }

    fn draw_table(&self, table_geo: &TableGeo) -> (impl IntoView, Signal<Vec<PointLabel<T>>>) {
        todo!()
    }

    fn draw_overlay(&self, size: Size, table_geo: &TableGeo, point_labels: Signal<Vec<PointLabel<T>>>) -> impl IntoView {
        todo!()
    }
}

impl<T, S> CalGraphDraw<T> for CalGraph<S>
where
    S: StoreField<Value = BasicTimeCell<T>> + Copy,
    T: Send + Sync + 'static
{
    fn draw_bg(&self, size: Size) -> (impl IntoView, TableGeo) {
        let Size {width, height} = size;
        let bg = BasicBg(
            BasicBgProps::builder()
                .class("col-start-1 row-start-1")
                .width(width)
                .height(height)
                .build(),
        );

        let table_x_margin: u32 = 30;
        let table_width = Signal::derive(move || width() - 2 * table_x_margin);
        let table_height = Signal::derive(move || height() - 30);
        let table_size = Size {width: table_width, height: table_height};

        (
            bg,
            TableGeo {
                offset: Offset {x: table_x_margin, y: 0},
                size: table_size,
            },
        )
    }

    fn draw_table(&self, table_geo: &TableGeo) -> (impl IntoView, Signal<Vec<PointLabel<T>>>) {
        let (width, height) = table_geo.size;
        let (table, points) = BasicTable(
            BasicTableProps::builder()
                .time_cell(self.time_cell)
                .class(format!(
                    "col-start-1 row-start-1 mx-[{}px] my-[{}px]",
                    table_geo.offset.0, table_geo.offset.1
                ))
                .width(width)
                .height(height)
                .build(),
        );
        (table, points)
    }

    fn draw_overlay(
        &self,
        table_geo: &TableGeo,
        point_labels: &Vec<PointLabel<T>>,
    ) -> impl IntoView {
        todo!()
    }
}
