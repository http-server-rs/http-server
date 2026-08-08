use leptos::prelude::*;

use file_explorer_ui::App;

fn main() {
    mount_to_body(|| {
        view! { <App /> }
    })
}
