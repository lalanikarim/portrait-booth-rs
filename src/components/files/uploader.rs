use leptos::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::JsFuture;
use web_sys::{DragEvent, Request, RequestInit, RequestMode};

use crate::models::{order::Order, order_item::OrderItem, user_order::UserOrder};

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::to_server_fn_error;
    }
}

#[server(GetPreSignedPutUrl, "/api")]
pub async fn get_pre_signed_put_url(cx: Scope, path: String) -> Result<String, ServerFnError> {
    crate::server::storage::create_presigned_put_url(path).await
}

#[server(GetUserOrder, "/api")]
pub async fn get_user_order(cx: Scope, id: u64) -> Result<UserOrder, ServerFnError> {
    match crate::pool(cx) {
        Ok(pool) => UserOrder::get_by_order_id(id, &pool).await,
        Err(e) => Err(e),
    }
}

#[server(AddOrderItemRequest, "/api")]
pub async fn add_order_item_request(
    cx: Scope,
    order: Order,
    mode: UploaderMode,
    file_name: String,
    mime_type: String,
) -> Result<OrderItem, ServerFnError> {
    let prefix = format!("/{:0>6}/{:?}", order.id, mode).to_lowercase();
    let file_split = file_name.split('.');
    let Some(ext) = file_split.last() else {
        return Err(ServerFnError::Args("Invalid File Name".to_string()));
    };
    let file_name = format!(
        "{}.{}",
        uuid::Uuid::new_v4().as_hyphenated().to_string(),
        ext
    );
    match crate::pool(cx) {
        Err(e) => Err(e),
        Ok(pool) => match get_remaining_uploads(cx, order.clone(), mode).await {
            Err(e) => Err(to_server_fn_error(e)),
            Ok(0) => Err(ServerFnError::ServerError(
                "No more uploads allowed".to_string(),
            )),
            Ok(_) => {
                match crate::server::storage::create_presigned_url_pair(
                    prefix,
                    file_name.clone(),
                    mime_type,
                )
                .await
                {
                    Err(e) => Err(e),
                    Ok((get_url, put_url)) => {
                        order
                            .add_order_item(file_name.clone(), mode, get_url, put_url, &pool)
                            .await
                    }
                }
            }
        },
    }
}

#[server(GetRemainingUploads, "/api")]
pub async fn get_remaining_uploads(
    cx: Scope,
    order: Order,
    mode: UploaderMode,
) -> Result<u64, ServerFnError> {
    match crate::pool(cx) {
        Err(e) => Err(e),
        Ok(pool) => order.remaining_order_items(mode, &pool).await,
    }
}

#[server(UpdateOrderUploadStatus, "/api")]
pub async fn update_order_upload_status(
    cx: Scope,
    order: Order,
    mode: UploaderMode,
) -> Result<u64, ServerFnError> {
    match crate::pool(cx) {
        Err(e) => Err(e),
        Ok(pool) => order.set_uploaded_for_zero_remaining(mode, &pool).await,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum FileUploadState {
    Added,
    Uploading,
    Done,
    Error,
}

pub fn get_mime_type(file_name: String) -> Result<String, ServerFnError> {
    let splits: Vec<&str> = file_name.split('.').collect();
    let Some(ext) = splits.last() else {
        return Err(ServerFnError::Args("Invalid file name received.".to_string()));
    };
    let ext = ext.to_lowercase();
    let ext = ext.as_str();

    let mime_type = match ext {
        "jpg" => "image/jpeg",
        "jpeg" => "image/jpeg",
        "png" => "image/png",
        _ => "application/octet-stream",
    };
    Ok(mime_type.to_string())
}

pub type UploaderMode = crate::models::order_item::Mode;

#[component]
pub fn Uploader(
    cx: Scope,
    order: Order,
    mode: UploaderMode,
    order_resource: Resource<Option<UserOrder>, Result<Option<Order>, ServerFnError>>,
) -> impl IntoView {
    let set_order = use_context::<WriteSignal<Option<UserOrder>>>(cx)
        .expect("Set Order Search should be present");
    let (files_to_upload, set_files_to_upload) =
        create_signal(cx, Vec::<(String, FileUploadState)>::new());
    let upload_file = move |file: web_sys::File, url: String| async move {
        let form_data = web_sys::FormData::new().expect("Form Data should create");
        form_data
            .append_with_blob_and_filename(&file.name(), &file, &file.name())
            .expect("Form Data append should work");
        form_data
            .set_with_str("content-type", &file.type_())
            .expect("Form Data set with str should work");
        let mut opts = RequestInit::new();
        opts.method("PUT");
        opts.mode(RequestMode::Cors);
        let buffer = JsFuture::from(file.array_buffer())
            .await
            .expect("JS Future for buffer should resolve");
        opts.body(Some(&buffer));
        let request =
            Request::new_with_str_and_init(&url, &opts).expect("Request init should work");
        let window = web_sys::window().expect("Window should work");
        _ = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request))
            .await
            .expect("JS Future await should work");
    };

    let on_drop = move |ev: DragEvent| {
        let mode = mode;
        let order = order.clone();
        ev.prevent_default();
        let dt = ev.data_transfer().unwrap();
        let items = dt.items();
        let c = items.length();
        let mut files = Vec::new();
        for i in 0..c {
            let item = items.get(i);
            if let Some(item) = item {
                if item.kind() == *"file" {
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
                let file_name = file.name();
                let Ok(mime_type) = get_mime_type(file_name.clone()) else {
                    log!("Invalid file extension");
                    return;
                };
                set_files_to_upload.update(|f| {
                    for elem in f.iter_mut() {
                        if elem.0 == file_name.clone() {
                            *elem = (file_name.clone(), FileUploadState::Uploading);
                        }
                    }
                });
                match add_order_item_request(cx, order.clone(), mode, file_name.clone(), mime_type)
                    .await
                {
                    Ok(OrderItem { put_url, .. }) => {
                        upload_file(file, put_url).await;
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
                };
                if let Ok(count) = update_order_upload_status(cx, order.clone(), mode).await {
                    if count > 0 {
                        if let Ok(user_order) = get_user_order(cx, order.clone().id).await {
                            set_order.set(Some(user_order));
                            order_resource.refetch();
                        };
                    }
                };
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
