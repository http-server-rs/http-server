use leptos::{component, view, For, IntoView, Signal, SignalGet};

use file_explorer_proto::BreadcrumbItem;

#[component]
pub fn NavigationBar(#[prop(into)] breadcrumbs: Signal<Vec<BreadcrumbItem>>) -> impl IntoView {
    view! {
        <div class="px-4 pb-4">
            <nav class="p-4 border w-full text-sm text-left rtl:text-right text-gray-600">
                <ol>
                    <For
                        each=move || breadcrumbs.get()
                        key=|bc| bc.entry_link.clone()
                        children=move |bc: BreadcrumbItem| {
                            view! {
                                <li class="inline">
                                    <a href={bc.entry_link.clone()}>{bc.entry_name.clone()}</a>
                                    <span class="mx-2">/</span>
                                </li>
                            }
                        }
                    />
                </ol>
            </nav>
        </div>
    }
}
