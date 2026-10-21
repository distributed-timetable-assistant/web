use leptos::prelude::codee::string::JsonSerdeCodec;
use leptos::prelude::*;
use leptos::server_fn::serde::Serialize;
use leptos::server_fn::serde::de::DeserializeOwned;
use leptos_use::storage::use_local_storage;
use reactive_stores::Store;

pub fn use_ctx<T>() -> Option<Store<T>>
where
    T: 'static,
{
    use_context::<Store<T>>()
}

/// init
pub fn get_stored_ctx<T>(key: &str) -> Store<T>
where
    T: PartialEq + Clone + Default + Send + Sync + DeserializeOwned + Serialize + 'static,
{
    let (state, set_state, _) = use_local_storage::<T, JsonSerdeCodec>(key);

    let init = Store::new(state.get_untracked());

    provide_context(init);

    Effect::new(move |_| {
        set_state.set(init.get());
    });

    init
}

pub fn get_ctx<T>() -> Store<T>
where
    T: Default + Send + Sync + 'static,
{
    let init = Store::new(T::default());
    provide_context(init);
    init
}