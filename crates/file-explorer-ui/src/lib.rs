use leptos::{component, spawn_local, view, IntoView};
use leptos_meta::provide_meta_context;
use rust_embed::Embed;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    spawn_local(async move {
        leptos::logging::warn!("Performing a request to the server");
        reqwest::get("http://localhost:3000/api/v1").await.unwrap();
    });

    view! {
        <div>
            <h1>App</h1>
        </div>
    }
}

#[derive(Embed)]
#[folder = "public/dist"]
pub struct Assets;
