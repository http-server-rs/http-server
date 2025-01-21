use leptos::{component, view, Children, IntoView};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Primary,
    #[default]
    Secondary,
}

#[component]
pub fn Button(children: Children) -> impl IntoView {
    view! {
        <button class="border font-semibold rounded-md px-4 py-2 flex justify-center items-center">
            {children()}
        </button>
    }
}
