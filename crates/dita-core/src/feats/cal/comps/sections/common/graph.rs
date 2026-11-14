use leptos::html::{Div, div};
use leptos::prelude::*;
use leptos_ui::tw_merge::tw_merge;
use leptos_use::use_resize_observer;
use models::cal::time_cells::common::Cell;
use reactive_stores::StoreField;

pub trait StoreCell: StoreField<Value = Self::Cell> + Clone + Track + Send + 'static {
    type Cell;
}
pub trait Time: Send + Sync + 'static {}

#[derive(Clone)]
pub struct CalGraph<S, C>
where
    S: StoreField<Value = C>,
    C: Cell,
{
    pub time_cell: S,
}

pub trait CalGraphDraw<S, C, T>: Clone
where
    S: StoreCell<Cell = C>,
    C: Cell,
    T: Time,
{
    fn draw(self, class: impl Into<String>) -> impl IntoView {
        let parent_ref = NodeRef::<Div>::new();
        let size = make_react(parent_ref);

        let (bg, table_geo) = self.clone().draw_bg(size);
        let table = self.clone().draw_table(table_geo);
        let overlay = self.draw_overlay(size, table_geo);

        let class = class.into();
        div()
            .node_ref(parent_ref)
            .class(tw_merge!("grid", class))
            .child((bg, table, overlay))
    }

    fn draw_bg(self, size: Size) -> (impl IntoView, TableGeo);

    fn draw_table(self, table_geo: TableGeo) -> impl IntoView;

    fn draw_overlay(self, size: Size, table_geo: TableGeo) -> impl IntoView;
}

#[derive(Debug, Clone, Copy)]
pub struct Size {
    pub width: Signal<u32>,
    pub height: Signal<u32>,
}

#[derive(Debug, Clone, Copy)]
pub struct Offset {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct TableGeo {
    pub offset: Offset,
    pub size: Size,
}

#[derive(Debug, Clone, Copy)]
pub struct RelativeLocation {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct PointLabel<T>
where
    T: Send + Sync + 'static,
{
    pub loc: RelativeLocation,
    pub time: T,
}

fn make_react(parent_ref: NodeRef<Div>) -> Size {
    let (width, set_width) = signal(1000);
    let (height, set_height) = signal(1000);

    use_resize_observer(parent_ref, move |entries, _| {
        if let Some(entry) = entries.first() {
            let rect = entry.content_rect();

            set_width.set(rect.width() as u32);
            set_height.set(rect.height() as u32);
        }
    });

    Size {
        width: width.into(),
        height: height.into(),
    }
}
