mod download_button;
mod entry;
mod entry_icon;

use leptos::{component, view, For, IntoView, Signal, SignalGet};

use file_explorer_proto::DirectoryEntry;

use self::entry::Entry;

#[component]
pub fn FileList(#[prop(into)] entries: Signal<Vec<DirectoryEntry>>) -> impl IntoView {
    view! {
        <div class="relative overflow-x-auto p-4">
            <table class="border-t border-x w-full text-sm text-left rtl:text-right text-gray-600">
                <thead class="border-b text-gray-700 bg-gray-50">
                    <tr>
                        <th scope="col" class="px-6 py-3 w-10" />
                        <th scope="col" class="px-6 py-3">
                            "Name"
                        </th>
                        <th scope="col" class="px-6 py-3">
                            "Size"
                        </th>
                        <th scope="col" class="px-6 py-3">
                            "Created"
                        </th>
                        <th scope="col" class="px-6 py-3">
                            "Modified"
                        </th>
                    </tr>
                </thead>
                <tbody class="text-gray-900 font-regular">
                   <For
                     each=move || entries.get()
                     key=|counter| counter.entry_path.clone()
                     children=move |dir_entry: DirectoryEntry| {
                        view! {
                            <Entry
                                name={dir_entry.display_name}
                                size={dir_entry.size_bytes}
                                entry_type={dir_entry.entry_type}
                                entry_path={dir_entry.entry_path}
                                date_created={dir_entry.date_created}
                                date_modified={dir_entry.date_modified}
                            />
                        }
                     }
                   />
                </tbody>
            </table>
        </div>
    }
}
