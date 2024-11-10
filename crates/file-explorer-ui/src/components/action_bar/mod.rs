mod file_upload;

use leptos::{component, view, IntoView};

use self::file_upload::FileUpload;

#[component]
pub fn ActionBar() -> impl IntoView {
    view! {
        <div class="px-4 py-4 border-t border-x w-full text-sm text-left rtl:text-right text-gray-600">
            <FileUpload />
        </div>
    }
}
