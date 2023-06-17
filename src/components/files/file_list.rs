use leptos::*;

#[server(GetFiles, "/api")]
pub async fn get_files(cx: Scope, prefix: String) -> Result<Vec<String>, ServerFnError> {
    crate::server::storage::get_files(prefix).await
}

#[component]
pub fn FileList(cx: Scope) -> impl IntoView {
    let (prefix, set_prefix) = create_signal(cx, "/".to_string());
    let get_files_resource = create_resource(
        cx,
        || (),
        move |_| async move { get_files(cx, prefix.get_untracked()).await },
    );
    create_effect(cx, move |_| {
        _ = prefix.get();
        get_files_resource.refetch();
    });
    view! { cx,
        <div class="container">
            <h2 class="header">"Files"</h2>
            <Suspense fallback=move || {
                view! { cx, <div>"Loading..."</div> }
            }>
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
                                files
                                    .iter()
                                    .map(|file| {
                                    let file = file.to_owned();
                                    let file1 = file.clone();
                                        view! { cx,
                                            <div on:click=move |_| {
                                                if file1.ends_with("/") {
                                                    log!("Choose: {}",file1);
                                                    set_prefix.update(|p| *p = file1.to_string());
                                                }
                                            }>{file}</div>
                                        }
                                    })
                                    .collect_view(cx)
                            }
                        }
                    }
                }}
            </Suspense>
        </div>
    }
}
