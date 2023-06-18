use leptos::*;

use crate::{components::files::uploader::UploaderMode, models::order::Order};

#[server(GetFiles, "/api")]
pub async fn get_files(cx: Scope, prefix: String) -> Result<Vec<String>, ServerFnError> {
    crate::server::storage::get_files(prefix).await
}

#[server(GetUrlRequest, "/api")]
pub async fn get_url_request(cx: Scope, path: String) -> Result<String, ServerFnError> {
    use crate::server::storage::create_presigned_url;
    create_presigned_url(path).await
}

#[component]
pub fn FileList(cx: Scope, order: Order, mode: UploaderMode) -> impl IntoView {
    let prefix = format!("/{:0>6}/{:?}/", order.id, mode).to_lowercase();
    let (prefix, set_prefix) = create_signal(cx, prefix);
    let (file, set_file) = create_signal(cx, None);
    let get_files_resource = create_resource(
        cx,
        || (),
        move |_| async move { get_files(cx, prefix.get_untracked()).await },
    );
    let open_file = move |file_name: String| {
        if file_name.ends_with("/") {
            log!("Dir: {}", file_name);
            set_prefix.update(|p| *p = file_name.to_string());
        } else {
            log!("File: {}", file_name);
            set_file.update(|f| *f = Some(file_name));
        }
    };
    create_effect(cx, move |_| {
        _ = prefix.get();
        get_files_resource.refetch();
    });
    create_effect(cx, move |_| {
        let file_name = file.get();
        if let Some(file_name) = file_name {
            spawn_local(async move {
                match get_url_request(cx, file_name).await {
                    Err(e) => error!("Error: {}", e.to_string()),
                    Ok(url) => log!("Url: {}", url),
                }
            });
        }
    });
    view! { cx,
        <div class="container">
            <h2 class="header">"Files"</h2>
            <Suspense fallback=move || {
                view! { cx, <div>"Loading..."</div> }
            }><div class="left-justified">
                {move || match get_files_resource.read(cx) {
                    None => {
                        view! { cx, <div>"Loading..."</div> }
                            .into_view(cx)
                    }
                    Some(files) => {
                        match files {
                            Err(e) => {
                                view! { cx, <div>{e.to_string()}</div> }
                                    .into_view(cx)
                            }
                            Ok(files) => {
                            if files.is_empty() {
                                view!{cx,<div class="text-center w-full">"Not files uploaded"</div>}.into_view(cx)
                            } else {
                                files
                                    .iter()
                                    .map(|file| {
                                    let file = file.to_owned();
                                    let file1 = file.clone();
                                        view! { cx,
                                            <div on:click=move |_| {
                                                open_file(file1.clone());
                                            }>{file}</div>
                                        }
                                    })
                                    .collect_view(cx)
                            }
                            }
                        }
                    }
                }}</div>
            </Suspense>
        </div>
    }
}
