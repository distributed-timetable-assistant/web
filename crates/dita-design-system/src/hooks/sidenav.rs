use leptos::logging;
use leptos::prelude::*;
use reactive_stores::{Store};
use serde::{Deserialize, Serialize};
use dita_state::app_state::StateProvider;
use dita_state::persist_state::{init_ctx, use_ctx, provide};

const LOCALSTORAGE_KEY: &str = "sidenav";

#[derive(Debug, Clone, Copy, Default, Store, Serialize, Deserialize, PartialEq)]
struct SidenavContext {
    pub open: bool,
}

#[derive(Clone, Copy)]
pub struct SidenavMode {
    ctx: Store<SidenavContext>,
}

impl SidenavMode {
    pub fn new() -> Self {
        Self {
            ctx: use_ctx()
                .unwrap_or_else(|| {
                    logging::warn!("Use Context: FAILED. SidenavContext not initialized at root component, build app state with sidenav state");
                    init_ctx(LOCALSTORAGE_KEY)
                }),
        }
    }
    pub fn toggle(self) {
        self.ctx.open().update(|v| *v = !*v);
    }

    pub fn is_open(&self) -> Signal<bool> {
        self.ctx.open().into()
    }
}

impl StateProvider for SidenavMode {
    fn provide() {
        provide::<SidenavContext>(LOCALSTORAGE_KEY);
    }
}