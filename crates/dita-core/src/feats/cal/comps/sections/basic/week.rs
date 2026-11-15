use crate::feats::cal::comps::sections::basic::bg::{BasicBg, BasicBgProps};
use crate::feats::cal::comps::sections::basic::overlay::{BasicOverlay, BasicOverlayProps};
use crate::feats::cal::comps::sections::basic::table::{BasicTable, BasicTableProps};
use crate::feats::cal::comps::sections::common::graph::{CalGraph, CalGraphDraw, Offset, PointLabel, Size, StoreCell, TableGeo};
use leptos::IntoView;
use leptos::prelude::Signal;
use models::cal::time_cells::basic::BasicTimeCell;
use models::cal::time_cells::weekly::WeekTime;

impl<S> CalGraphDraw<S, BasicTimeCell<WeekTime>, WeekTime> for CalGraph<S, BasicTimeCell<WeekTime>>
where
    S: StoreCell<Cell = BasicTimeCell<WeekTime>>,
{
    fn draw_bg(&self, size: Size) -> (impl IntoView + use<S>, TableGeo) {
        let Size { width, height } = size;
        let bg = BasicBg(
            BasicBgProps::builder()
                .class("col-start-1 row-start-1")
                .width(width.clone())
                .height(height)
                .build(),
        );

        let table_x_margin: u32 = 30;
        let table_width = Signal::derive(move || width() - 2 * table_x_margin);
        let table_height = Signal::derive(move || height() - 30);
        let table_size = Size {
            width: table_width,
            height: table_height,
        };

        (
            bg,
            TableGeo {
                offset: Offset {
                    x: table_x_margin,
                    y: 0,
                },
                size: table_size,
            },
        )
    }

    fn draw_table(&self, table_geo: TableGeo) -> impl IntoView + use<S> {
        let Size { width, height } = table_geo.size;
        let Offset { x, y } = table_geo.offset;
        BasicTable(
            BasicTableProps::builder()
                .time_cell(self.time_cell)
                .class(format!("col-start-1 row-start-1 mx-[{x}px] my-[{y}px]"))
                .width(width)
                .height(height)
                .build(),
        )
    }

    fn draw_overlay(&self, size: Size, table_geo: TableGeo) -> impl IntoView + use<S> {
        let Size { width, height } = size;
        let Size {
            width: table_width,
            height: table_height,
        } = table_geo.size;
        let Offset {
            x: table_x,
            y: table_y,
        } = table_geo.offset;
        let points: Vec<PointLabel<WeekTime>> = vec![];
        BasicOverlay(
            BasicOverlayProps::builder()
                .class("col-start-1 row-start-1")
                .width(width)
                .height(height)
                .table_x_offset(table_x)
                .table_y_offset(table_y)
                .table_width(table_width)
                .table_height(table_height)
                .points(points)
                .build(),
        )
    }
}
