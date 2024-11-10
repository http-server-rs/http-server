mod api;
mod components;

use gloo::utils::window;
use leptos::{
    component, create_memo, create_signal, spawn_local, view, IntoView, SignalGet, SignalSet,
};
use leptos_meta::provide_meta_context;
use rust_embed::Embed;

use file_explorer_proto::DirectoryIndex;

use self::api::Api;
use self::components::action_bar::ActionBar;
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
        let Ok(pathname) = window().location().pathname() else {
            leptos::logging::error!("Failed to get the pathname");
            return;
        };

        let Ok(index) = Api::new().peek(&pathname).await else {
            leptos::logging::error!("Failed to fetch the directory index");
            return;
        };

        index_setter.set(Some(index));
    });

    view! {
        <div>
            <ActionBar />
            <FileList entries={entries} />
        </div>
    }
}
