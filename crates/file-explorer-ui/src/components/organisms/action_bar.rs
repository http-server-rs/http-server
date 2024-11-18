use leptos::{component, view, IntoView};

use crate::components::molecules::file_upload::FileUpload;

#[component]
pub fn ActionBar() -> impl IntoView {
    view! {
        <div class="p-4 w-full text-sm text-left rtl:text-right text-gray-600">
            <FileUpload />
        </div>
    }
}
