use chrono::{DateTime, Local};
use leptos::{component, view, IntoView, Show};

use file_explorer_proto::EntryType;

use super::download_button::DownloadButton;
use super::entry_icon::EntryIcon;

#[component]
pub fn Entry(
    #[prop(into)] name: String,
    #[prop(into)] size: u64,
    #[prop(into)] is_dir: bool,
    #[prop(into)] entry_type: EntryType,
    #[prop(into)] entry_path: String,
    #[prop(into)] date_created: Option<DateTime<Local>>,
    #[prop(into)] date_modified: Option<DateTime<Local>>,
) -> impl IntoView {
    let download_name = name.clone();
    let format_date_or_default = |date: Option<DateTime<Local>>| {
        date.map(|d| d.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_else(|| "Unknown".to_string())
    };

    view! {
        <tr class="bg-white border-b hover:bg-blue-50 text-gray-600">
            <td scope="row" class="px-6 py-2 text-zinc-400">
                <EntryIcon entry_type={entry_type.clone()} />
            </td>
            <th scope="row" class="px-6 py-2 font-semibold whitespace-nowrap text-gray-800">
                <span class="flex items-center justify-between">
                    <a href={entry_path.clone()} class="hover:text-blue-500">
                        {name}
                    </a>
                    <Show when=move || !is_dir>
                        <DownloadButton
                            entry_path={entry_path.clone()}
                            download_name={download_name.clone()}
                        />
                    </Show>
                </span>
            </th>
            <th scope="row" class="px-6 py-2 font-mono  font-normal">
                {size}
            </th>
            <th scope="row" class="px-6 py-2 font-normal">
                {format_date_or_default(date_created)}
            </th>
            <th scope="row" class="px-6 py-2 font-normal">
                {format_date_or_default(date_modified)}
            </th>
        </tr>
    }
}
