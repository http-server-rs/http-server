use leptos::prelude::*;

use crate::api::proto::EntryType;
use crate::components::atoms::icons::{File, Folder, Git, Justfile, Markdown, Rust, Toml};

#[component]
pub fn EntryIcon(#[prop(into)] entry_type: EntryType) -> impl IntoView {
    let icon = match entry_type {
        EntryType::Directory => view! {
            <Folder />
        }
        .into_any(),
        EntryType::Git => view! {
            <Git />
        }
        .into_any(),
        EntryType::Justfile => view! {
            <Justfile />
        }
        .into_any(),
        EntryType::Markdown => view! {
            <Markdown />
        }
        .into_any(),
        EntryType::Rust => view! {
            <Rust />
        }
        .into_any(),
        EntryType::Toml => view! {
            <Toml />
        }
        .into_any(),
        _ => view! {
            <File />
        }
        .into_any(),
    };

    view! {
        <figure class="h-6 w-6">
            {icon}
        </figure>
    }
}
