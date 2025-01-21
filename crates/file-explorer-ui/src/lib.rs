mod api;
mod components;

use leptos::{component, view, IntoView};
use leptos_meta::provide_meta_context;

use self::components::pages::explorer::Explorer;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Explorer />
    }
}
