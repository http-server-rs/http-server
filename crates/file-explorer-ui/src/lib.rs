mod api;
mod components;

use leptos::{
    component, create_memo, create_signal, spawn_local, view, IntoView, SignalGet, SignalSet,
};
use leptos_meta::provide_meta_context;
use rust_embed::Embed;

use file_explorer_proto::DirectoryIndex;

use self::api::Api;
use self::components::file_list::FileList;

#[derive(Embed)]
#[folder = "public/dist"]
pub struct Assets;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let (index_getter, index_setter) = create_signal::<Option<DirectoryIndex>>(None);
    let entries = create_memo(move |_| {
        index_getter
            .get()
            .map(|index| index.entries.clone())
            .unwrap_or_default()
    });

    spawn_local(async move {
        leptos::logging::warn!("Performing a request to the server");
        let index = Api::new().peek("").await.unwrap();

        index_setter.set(Some(index));
    });

    view! {
        <div>
            <FileList entries={entries} />
        </div>
    }
}
