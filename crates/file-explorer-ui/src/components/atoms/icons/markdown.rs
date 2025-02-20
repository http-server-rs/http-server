use leptos::{component, view, IntoView};

#[component]
pub fn Markdown() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 32 32">
          <path fill="#42a5f5" d="m14 10-4 3.5L6 10H4v12h4v-6l2 2 2-2v6h4V10h-2zm12 6v-6h-4v6h-4l6 8 6-8h-4z"/>
        </svg>
    }
}
