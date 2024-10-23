use leptos::{mount_to_body, view};

use file_explorer_ui::App;

fn main() {
    mount_to_body(|| {
        view! { <App/> }
    })
}
