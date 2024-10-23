use leptos::{component, view, IntoView};
use leptos_meta::provide_meta_context;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <div>
            <h1>App</h1>
        </div>
    }
}
