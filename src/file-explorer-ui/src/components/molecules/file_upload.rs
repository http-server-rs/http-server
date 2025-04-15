use gloo::utils::window;
use leptos::logging::log;
use leptos::wasm_bindgen::JsCast;
use leptos::{component, create_action, create_node_ref, html, view, IntoView};
use web_sys::{Event, FormData, HtmlInputElement};

use crate::api::Api;
use crate::components::atoms::button::Button;

#[component]
pub fn FileUpload() -> impl IntoView {
    let file_input_el = create_node_ref::<html::Input>();
    let upload_file = create_action(|file: &web_sys::File| {
        let file = file.to_owned();

        let form_data = FormData::new().unwrap();
        form_data.append_with_blob("file", &file).unwrap();

        async move {
            match Api::new().upload(form_data).await {
                Ok(_) => {
                    log!("File uploaded successfully");
                    window()
                        .alert_with_message("File uploaded successfully")
                        .unwrap();
                }
                Err(e) => {
                    log!("Failed to upload file: {:?}", e);
                    window()
                        .alert_with_message("Failed to upload file")
                        .unwrap();
                }
            }
        }
    });

    let handle_button_click = {
        move |_| {
            file_input_el.get_untracked().unwrap().click();
        }
    };

    view! {
        <div>
            <Button on:click={handle_button_click}>"Upload a File"</Button>
            <input
                type="file"
                hidden="true"
                node_ref=file_input_el
                on:change=move |ev: Event| {
                    let el = ev.target().expect("Failed to retrieve target").unchecked_into::<HtmlInputElement>();
                    let mb_file_list = el.files();

                    if let Some(file_list) = mb_file_list {
                        if let Some(file) = file_list.get(0) {
                            log!("File selected: {:?}", file);
                            upload_file.dispatch(file);
                        } else {
                            log!("No file selected");
                        }
                    }
                }
            />
        </div>
    }
}
