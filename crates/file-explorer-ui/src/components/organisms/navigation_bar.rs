use leptos::{component, view, For, IntoView, Signal, SignalGet};

use file_explorer_proto::BreadcrumbItem;

use crate::components::atoms::icons::House;

#[component]
pub fn NavigationBar(#[prop(into)] breadcrumbs: Signal<Vec<BreadcrumbItem>>) -> impl IntoView {
    view! {
        <div class="px-4 pb-4">
            <nav class="p-4 border w-full text-sm text-left rtl:text-right text-gray-600">
                <ol class="flex items-center justify-start">
                    <For
                        each=move || breadcrumbs.get()
                        key=|bc| bc.entry_link.clone()
                        children=move |bc: BreadcrumbItem| {
                            if bc.depth == 0 {
                                view! {
                                    <li class="flex items-center justify-center">
                                        <a href={bc.entry_link.clone()}>
                                            <figure class="inline h-6 w-6">
                                                <House class="h-6 w-6" />
                                            </figure>
                                        </a>
                                        <span class="mx-2">/</span>
                                    </li>
                                }
                            } else {
                                view! {
                                    <li class="flex items-center justify-center">
                                        <a href={bc.entry_link.clone()}>{bc.entry_name.clone()}</a>
                                        <span class="mx-2">/</span>
                                    </li>
                                }
                            }
                        }
                    />
                </ol>
            </nav>
        </div>
    }
}
