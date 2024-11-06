use gloo_file::{Blob, ObjectUrl};
use leptos::{component, create_node_ref, html::A, spawn_local, view, IntoView};

use crate::{
    api::{Api, FileDownload},
    components::icons::Download,
};

#[component]
pub fn DownloadButton(#[prop(into)] entry_path: String) -> impl IntoView {
    let anchor_ref = create_node_ref::<A>();
    let download_file = {
        move |_: _| {
            let entry_path = entry_path.clone();
            spawn_local(async move {
                let api = Api::new();
                match api.download(&entry_path).await {
                    Ok(FileDownload { bytes, mime }) => {
                        let blob = Blob::new_with_options(bytes.as_slice(), Some(&mime));

                        let object_url = ObjectUrl::from(blob);

                        if let Some(anchor_el) = anchor_ref.get_untracked() {
                            anchor_el.set_href(&object_url);
                            anchor_el.set_download(&entry_path);
                            anchor_el.click();
                        }
                    }
                    Err(err) => {
                        leptos::logging::error!("Failed to download file: {:?}", err);
                    }
                }
            });
        }
    };

    view! {
        <button
            class="flex justify-center items-center h-6 w-6"
            on:click={download_file}
        >
            <Download />
        </button>
        <a hidden="true" _ref={anchor_ref} />
    }
}
