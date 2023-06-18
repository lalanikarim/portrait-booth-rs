use leptos::*;
use web_sys::{DragEvent, Request, RequestInit, RequestMode};

#[server(GetPreSignedPutUrl, "/api")]
pub async fn get_pre_signed_put_url(cx: Scope, path: String) -> Result<String, ServerFnError> {
    crate::server::storage::create_presigned_put_url(path).await
}

#[component]
pub fn Uploader(cx: Scope) -> impl IntoView {
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
        let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request))
            .await
            .expect("JS Future await should work");
        log!("{:#?}", resp_value);
    };
    let on_drop = move |ev: DragEvent| {
        ev.prevent_default();
        let dt = ev.data_transfer().unwrap();
        let items = dt.items();
        let c = items.length();
        for i in 0..c {
            let item = items.get(i);
            if let Some(item) = item {
                if item.kind() == "file".to_string() {
                    if let Ok(Some(file)) = item.get_as_file() {
                        log!("Uploading {}", file.name());
                        spawn_local(async move {
                            match get_pre_signed_put_url(cx, format!("/files/{}", file.name()))
                                .await
                            {
                                Ok(url) => {
                                    upload_file(file, url).await;
                                    log!("Upload done");
                                }
                                Err(e) => error!("{:#?}", e),
                            }
                        });
                    }
                }
            };
        }
    };
    view! { cx,
        <div class="container">
            <h2 class="header">"Upload Files"</h2>
            <div class="m-5 h-60 border-4 border-dashed rounded-xl">
                <div
                    class="flex items-center justify-center h-full"
                    on:dragover=move |ev| {
                        log!("Drag over");
                        ev.prevent_default();
                    }
                    on:drop=on_drop
                >
                    "Test"
                </div>
            </div>
        </div>
    }
}
