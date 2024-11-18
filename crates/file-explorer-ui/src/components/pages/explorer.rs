use gloo::utils::window;
use leptos::{
    component, create_memo, create_signal, spawn_local, view, IntoView, SignalGet, SignalSet,
};

use file_explorer_proto::DirectoryIndex;

use crate::api::Api;
use crate::components::organisms::action_bar::ActionBar;
use crate::components::organisms::navigation_bar::NavigationBar;
use crate::components::templates::file_list::FileList;

#[component]
pub fn Explorer() -> impl IntoView {
    let (index_getter, index_setter) = create_signal::<Option<DirectoryIndex>>(None);
    let entries = create_memo(move |_| {
        index_getter
            .get()
            .map(|index| index.entries.clone())
            .unwrap_or_default()
    });
    let breadcrumbs = create_memo(move |_| {
        index_getter
            .get()
            .map(|index| index.breadcrumbs.clone())
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
            <NavigationBar breadcrumbs={breadcrumbs} />
            <FileList entries={entries} />
        </div>
    }
}
