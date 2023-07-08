use leptos::{html::Dialog, *};
use web_sys::MouseEvent;

use crate::{
    components::{
        files::uploader::UploaderMode,
        util::{empty_view::EmptyView, loading::Loading},
    },
    models::{order::Order, order_item::OrderItem},
};

#[server(GetFiles, "/api")]
pub async fn get_files(cx: Scope, prefix: String) -> Result<Vec<String>, ServerFnError> {
    crate::server::storage::get_files(prefix).await
}

#[server(GetOrderItems, "/api")]
pub async fn get_order_items(
    cx: Scope,
    order: Order,
    mode: UploaderMode,
) -> Result<Vec<OrderItem>, ServerFnError> {
    match crate::pool(cx) {
        Err(e) => Err(e),
        Ok(pool) => order.get_order_items(mode, &pool).await,
    }
}

#[server(GetUrlRequest, "/api")]
pub async fn get_url_request(cx: Scope, path: String) -> Result<String, ServerFnError> {
    use crate::server::storage::create_presigned_url;
    create_presigned_url(path).await
}

#[server(DeleteOrderItemRequest, "/api")]
pub async fn delete_order_item_request(
    cx: Scope,
    order: Order,
    order_item: OrderItem,
) -> Result<u64, ServerFnError> {
    let mode = order_item.mode;
    match crate::pool(cx) {
        Err(e) => Err(e),
        Ok(pool) => {
            let prefix = format!("/{:0>6}/{:?}", order.id, mode).to_lowercase();
            let path = format!("{prefix}/{}", order_item.file_name);
            match crate::server::storage::delete_file(path).await {
                Err(e) => Err(e),
                Ok(_) => match OrderItem::delete(order_item.id, &pool).await {
                    Err(e) => Err(e),
                    Ok(_) => order.revert_uploaded_status(mode, &pool).await,
                },
            }
        }
    }
}

#[component]
pub fn FileList(cx: Scope, order: Order, mode: UploaderMode) -> impl IntoView {
    let (to_delete, set_to_delete) = create_signal::<Option<OrderItem>>(cx, None);
    let delete_order_item_action = create_server_action::<DeleteOrderItemRequest>(cx);
    let order_d = order.clone();
    let get_order_items = create_resource(
        cx,
        move || delete_order_item_action.version().get(),
        move |_| {
            let order = order.clone();
            async move { get_order_items(cx, order, mode).await }
        },
    );
    let delete_dialog = create_node_ref::<Dialog>(cx);
    view! { cx,
        <div class="container">
            <h2 class="header">"Files"</h2>
            <Suspense fallback=move || {
                view! { cx, <Loading/> }
            }>
                <div class="flex flex-wrap">
                    {move || match get_order_items.read(cx) {
                        None => view! { cx, <Loading/> }.into_view(cx),
                        Some(order_items) => {
                            match order_items {
                                Err(e) => view! { cx, <div>{e.to_string()}</div> }.into_view(cx),
                                Ok(order_items) => {
                                    if order_items.is_empty() {
                                        view! { cx,
                                            <div class="text-center w-full">"Not files uploaded"</div>
                                        }
                                            .into_view(cx)
                                    } else {
                                        order_items
                                            .into_iter()
                                            .map(|order_item| {
                                                let delete_dialog = delete_dialog
                                                    .get()
                                                    .expect("Delete Dialog should be present");
                                                let get_url = order_item.clone().get_url;
                                                let delete_click = move |_: MouseEvent| {
                                                    set_to_delete.set(Some(order_item.clone()));
                                                    _ = delete_dialog.show_modal();
                                                };
                                                view! { cx,
                                                    <div class="w-48 p-2">
                                                        <a href=&get_url>
                                                            <img src=&get_url/>
                                                        </a>
                                                    </div>
                                                    <div class="w-48 p-2">
                                                        <button on:click=delete_click>"Delete"</button>
                                                    </div>
                                                }
                                            })
                                            .collect_view(cx)
                                    }
                                }
                            }
                        }
                    }}
                </div>
                <dialog _ref=delete_dialog>
                    <div class="flex flex-col justify-items-center gap-y-5">
                        {
                            let order_d = order_d.clone();
                            move || {
                                let order_d = order_d.clone();
                                match to_delete.get() {
                                    None => view! { cx, <EmptyView/> },
                                    Some(order_item) => {
                                        let get_url = order_item.get_url.clone();
                                        let delete_dialog = delete_dialog
                                            .get()
                                            .expect("Delete dialog should be present");
                                        let delete_click = move |_: MouseEvent| {
                                            delete_order_item_action
                                                .dispatch(DeleteOrderItemRequest {
                                                    order: order_d.clone(),
                                                    order_item: order_item.clone(),
                                                });
                                            delete_dialog.close();
                                        };
                                        view! { cx,
                                            <div class="text-lg">"Confirm delete"</div>
                                            <img src=get_url/>
                                            <button class="red" on:click=delete_click>"Delete"</button>
                                        }
                                            .into_view(cx)
                                    }
                                }
                            }
                        }
                        <button on:click=move |_| {
                            let delete_dialog = delete_dialog
                                .get()
                                .expect("Delete dialog should be present");
                            delete_dialog.close();
                        }>"Cancel"</button>
                    </div>
                </dialog>
            </Suspense>
        </div>
    }
}
