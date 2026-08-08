use leptos::children::Children;
use leptos::prelude::*;

#[component]
pub fn Button(children: Children) -> impl IntoView {
    view! {
        <button class="border font-semibold rounded-md px-4 py-2 flex justify-center items-center">
            {children()}
        </button>
    }
}
