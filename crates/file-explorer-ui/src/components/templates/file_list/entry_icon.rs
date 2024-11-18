use leptos::{component, view, IntoView};

use file_explorer_proto::EntryType;

use crate::components::atoms::icons::{File, Folder, Git, Justfile, Markdown, Rust, Toml};

#[component]
pub fn EntryIcon(#[prop(into)] entry_type: EntryType) -> impl IntoView {
    let icon = match entry_type {
        EntryType::Directory => view! {
            <Folder />
        },
        EntryType::Git => view! {
            <Git />
        },
        EntryType::Justfile => view! {
            <Justfile />
        },
        EntryType::Markdown => view! {
            <Markdown />
        },
        EntryType::Rust => view! {
            <Rust />
        },
        EntryType::Toml => view! {
            <Toml />
        },
        _ => view! {
            <File />
        },
    };

    view! {
        <figure class="h-6 w-6">
            {icon}
        </figure>
    }
}
