use leptos::*;
use serde::{Deserialize, Serialize};
use web_sys::{DragEvent, Request, RequestInit, RequestMode};

use crate::models::order::Order;

#[server(GetPreSignedPutUrl, "/api")]
pub async fn get_pre_signed_put_url(cx: Scope, path: String) -> Result<String, ServerFnError> {
    crate::server::storage::create_presigned_put_url(path).await
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum FileUploadState {
    Added,
    Uploading,
    Done,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum UploaderMode {
    Original,
    Thumbnail,
    Processed,
}

#[component]
pub fn Uploader(cx: Scope, order: Order, mode: UploaderMode) -> impl IntoView {
    let (files_to_upload, set_files_to_upload) =
        create_signal(cx, Vec::<(String, FileUploadState)>::new());
    let upload_file = move |file: web_sys::File, url: String| async move {
        let form_data = web_sys::FormData::new().expect("Form Data should create");
        form_data
            .append_with_blob_and_filename(&file.name(), &file, &file.name())
            .expect("Form Data append should work");
        let mut opts = RequestInit::new();
        opts.method("PUT");
        opts.mode(RequestMode::Cors);
        opts.body(Some(&form_data));
        let request =
            Request::new_with_str_and_init(&url, &opts).expect("Request init should work");
        let window = web_sys::window().expect("Window should work");
        _ = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request))
            .await
            .expect("JS Future await should work");
    };
    let on_drop = move |ev: DragEvent| {
        let mode = mode.clone();
        ev.prevent_default();
        let dt = ev.data_transfer().unwrap();
        let items = dt.items();
        let c = items.length();
        let mut files = Vec::new();
        for i in 0..c {
            let item = items.get(i);
            if let Some(item) = item {
                if item.kind() == "file".to_string() {
                    if let Ok(Some(file)) = item.get_as_file() {
                        set_files_to_upload.update(|f| {
                            f.push((file.name(), FileUploadState::Added));
                        });
                        files.push(file);
                    }
                }
            }
        }
        spawn_local(async move {
            for file in files.into_iter() {
                let prefix = format!("/{:0>6}/{:?}", order.id, mode).to_lowercase();
                let full_file_name = format!("{}/{}", prefix, &file.name());
                let file_name = file.name();
                set_files_to_upload.update(|f| {
                    for elem in f.iter_mut() {
                        if elem.0 == file_name.clone() {
                            *elem = (file_name.clone(), FileUploadState::Uploading);
                        }
                    }
                });
                match get_pre_signed_put_url(cx, full_file_name).await {
                    Ok(url) => {
                        upload_file(file, url).await;
                        set_files_to_upload.update(|f| {
                            for elem in f.iter_mut() {
                                if elem.0 == file_name.clone() {
                                    *elem = (file_name.clone(), FileUploadState::Done);
                                }
                            }
                        });
                    }
                    Err(e) => {
                        error!("{:#?}", e);
                        set_files_to_upload.update(|f| {
                            for elem in f.iter_mut() {
                                if elem.0 == file_name.clone() {
                                    *elem = (file_name.clone(), FileUploadState::Error);
                                }
                            }
                        });
                    }
                }
            }
        });
    };
    let file_list = move || {
        files_to_upload
            .get()
            .iter()
            .map(move |(file, state)| {
                let state = state.to_owned();
                view! { cx,
                    <div>
                        <span>{file}</span>
                        {move || match state {
                            FileUploadState::Added => {
                                view! { cx, <span class="italic text-stone-400">" added"</span> }
                            }
                            FileUploadState::Uploading => {
                                view! { cx, <span class="italic text-green-400">" uploading"</span> }
                            }
                            FileUploadState::Done => {
                                view! { cx, <span class="italic text-green-400">" done"</span> }
                            }
                            FileUploadState::Error => {
                                view! { cx, <span class="italic text-red-400">" error"</span> }
                            }
                        }}
                    </div>
                }
            })
            .collect::<Vec<_>>()
    };
    view! { cx,
        <div class="container">
            <h2 class="header">"Upload Files"</h2>
            <div class="m-2 h-60 border-4 border-dashed rounded-xl">
                <div
                    class="left-justified overflow-y-scroll h-full"
                    on:dragover=move |ev| {
                        ev.prevent_default();
                    }
                    on:drop=on_drop
                >
                    {file_list}
                </div>
            </div>
        </div>
    }
}
