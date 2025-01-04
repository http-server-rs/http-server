mod api;
mod components;

use leptos::{component, view, IntoView};
use leptos_meta::provide_meta_context;
use rust_embed::Embed;

use crate::components::pages::explorer::Explorer;

#[derive(Embed)]
#[folder = "public/dist"]
pub struct Assets;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Explorer />
    }
}
