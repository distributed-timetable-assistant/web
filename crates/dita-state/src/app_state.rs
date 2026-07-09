pub struct AppState;

impl AppState {
    pub fn builder() -> AppStateBuilder {
        AppStateBuilder {
            providers: Vec::new(),
        }
    }
}

pub struct AppStateBuilder {
    providers: Vec<Box<dyn FnOnce()>>,
}

impl AppStateBuilder {
    pub fn with<P: StateProvider + 'static>(mut self) -> Self {
        self.providers.push(Box::new(P::provide));
        self
    }
    pub fn build(self) -> AppState {
        self.providers.into_iter().for_each(|provider| provider());
        AppState
    }
}

pub trait StateProvider {
    fn provide();
}