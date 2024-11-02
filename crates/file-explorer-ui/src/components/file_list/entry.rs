use chrono::{DateTime, Local};
use leptos::{component, view, IntoView};

use file_explorer_proto::EntryType;

use super::entry_icon::EntryIcon;

#[component]
pub fn Entry(
    #[prop(into)] name: String,
    #[prop(into)] size: u64,
    #[prop(into)] entry_type: EntryType,
    #[prop(into)] date_created: Option<DateTime<Local>>,
    #[prop(into)] date_modified: Option<DateTime<Local>>,
) -> impl IntoView {
    let format_date_or_default = |date: Option<DateTime<Local>>| {
        date.map(|d| d.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_else(|| "Unknown".to_string())
    };

    view! {
        <tr class="bg-white border-b hover:bg-blue-50 text-gray-600">
            <td scope="row" class="px-6 py-2 text-zinc-400">
                <EntryIcon entry_type={entry_type} />
            </td>
            <th scope="row" class="px-6 py-2 font-semibold whitespace-nowrap text-gray-800">
                {name}
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
