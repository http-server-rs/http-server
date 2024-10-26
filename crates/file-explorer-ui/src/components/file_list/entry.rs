use leptos::{component, view, IntoView};

#[component]
pub fn Entry(#[prop(into)] name: String) -> impl IntoView {
    view! {
        <tr class="bg-white border-b hover:bg-blue-50">
            <td scope="row" class="px-6 py-4">
                <img src="http://via.placeholder.com/16" height="16" width="16" />
            </td>
            <th scope="row" class="px-6 py-4 font-semibold whitespace-nowrap">
                {name}
            </th>
            <th scope="row" class="px-6 py-4">
                "128.9 MB"
            </th>
            <th scope="row" class="px-6 py-4">
                "2024-10-11"
            </th>
            <th scope="row" class="px-6 py-4">
                "2024-10-11"
            </th>
        </tr>
    }
}
