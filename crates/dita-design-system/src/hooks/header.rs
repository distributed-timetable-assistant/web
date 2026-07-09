use dita_state::app_state::StateProvider;
use dita_state::persist_state::{init_ctx, provide, use_ctx};
use leptos::logging;
use leptos::prelude::*;
use reactive_stores::Store;
use serde::{Deserialize, Serialize};

const LOCALSTORAGE_KEY: &str = "breadcrumbs";

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Crumb {
    pub title: String,
    pub path: String,
}

#[derive(Debug, Clone, Default, Store, Serialize, Deserialize, PartialEq)]
struct HeaderContext {
    pub crumbs: Vec<Crumb>,
    pub page_title: String,
}

#[derive(Clone)]
pub struct HeaderMode {
    ctx: Store<HeaderContext>,
}

impl HeaderMode {
    pub fn new() -> Self {
        Self {
            ctx: use_ctx()
                .unwrap_or_else(|| {
                    logging::warn!("Use Context: FAILED. BreadcrumbsContext not initialized at root component, build app state with sidenav state");
                    init_ctx(LOCALSTORAGE_KEY)
                }),
        }
    }

    pub fn set_crumbs(&self, crumbs: Vec<Crumb>) {
        self.ctx.crumbs().set(crumbs);
    }

    pub fn get_crumbs(&self) -> Signal<Vec<Crumb>> {
        self.ctx.crumbs().into()
    }

    pub fn set_page_title(&self, title: String) {
        self.ctx.page_title().set(title);
    }

    pub fn get_page_title(&self) -> Signal<String> {
        self.ctx.page_title().into()
    }
}

impl StateProvider for HeaderMode {
    fn provide() {
        provide::<HeaderContext>(LOCALSTORAGE_KEY);
    }
}
