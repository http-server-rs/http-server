use gloo::utils::window;
use leptos::logging::log;
use leptos::wasm_bindgen::JsCast;
use leptos::{component, create_action, create_node_ref, html, view, IntoView};
use web_sys::{Event, FormData, HtmlInputElement};

#[component]
pub fn FileUpload() -> impl IntoView {
    let file_input_el = create_node_ref::<html::Input>();
    let upload_file = create_action(|file: &web_sys::File| {
        let file = file.to_owned();

        let form_data = FormData::new().unwrap();
        form_data.append_with_blob("file", &file).unwrap();

        async move {
            log!("Uploading file...");

            let response = gloo::net::http::Request::post(&format!(
                "{}/api/v1",
                &window().location().origin().unwrap()
            ))
            .body(form_data)
            .unwrap() // result can't be error
            .send()
            .await;

            match response {
                Ok(_) => {
                    log!("File successfully uploaded!");
                }
                Err(err) => {
                    log!("Error uploading file: {}", err);
                }
            }
        }
    });

    view! {
        <div>
            <input
                type="file"
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
