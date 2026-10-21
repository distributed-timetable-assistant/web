use crate::common::hooks::header::HeaderState;
use crate::feats::cal::comps::avail::{Avail, AvailProps};
use leptos::logging;
use leptos::prelude::*;
use models::cal::{Cal, CalStoreFields};
use reactive_stores::Store;

#[component]
pub fn CalPage() -> impl IntoView {
    let header_ctx = HeaderState::new();
    header_ctx.set_page_title("Calendar".into());
    let cal = get_cal();
    let avails = cal.avails();
    logging::log!("CalPage Initiate -> START: call=({:?})", cal.get());
    ForEnumerate(ForEnumerateProps {
        each: move || avails,
        key: |avail| avail.key(),
        children: move |index, avail| {
            logging::log!(
                "Cell Initiate -> START: index=({}), id=({})",
                index.get(),
                avail.key(),
            );
            Avail(AvailProps { avail })
        },
    })
}

fn get_cal() -> Store<Cal> {
    const YAML: &str = include_str!("../../../test/fixtures/cal/samples/complex.yaml");
    let cal: Cal = serde_yaml::from_str(YAML).unwrap();
    Store::new(cal)
}
