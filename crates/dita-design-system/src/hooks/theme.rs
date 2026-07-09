use dita_state::persist_state::{init_ctx, provide, use_ctx};
use leptos::logging;
use leptos::prelude::*;
use reactive_stores::Store;
use serde::{Deserialize, Serialize};
use dita_state::app_state::StateProvider;

const LOCALSTORAGE_KEY: &str = "theme";

#[derive(Debug, Clone, Copy, Default, Store, Serialize, Deserialize, PartialEq)]
struct ThemeContext {
    pub dark_mode: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct ThemeMode {
    ctx: Store<ThemeContext>,
}

impl ThemeMode {
    pub fn new() -> Self {
        let ctx: Store<ThemeContext> = use_ctx()
                .unwrap_or_else(|| {
                    logging::warn!("Use Context: FAILED. ThemeContext not initialized at root component, build app state with theme state");
                    init_ctx(LOCALSTORAGE_KEY)
                });

        Effect::new(move |_| {
            let theme = ctx.get();

            let document = web_sys::window().unwrap().document().unwrap();
            let html = document.document_element().unwrap();

            match theme.dark_mode {
                true => {
                    html.class_list().add_1("dark").unwrap();
                }
                false => {
                    html.class_list().remove_1("dark").unwrap();
                }
            }
        });

        Self { ctx }
    }

    pub fn toggle(&self) {
        self.ctx.dark_mode().update(|v| *v = !*v);
    }

    pub fn is_dark(&self) -> Signal<bool> {
        self.ctx.dark_mode().into()
    }
}

impl StateProvider for ThemeMode {
    fn provide() {
        provide::<ThemeContext>(LOCALSTORAGE_KEY);
    }
}
