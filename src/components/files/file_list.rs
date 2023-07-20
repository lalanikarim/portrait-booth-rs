use leptos::{html::Dialog, *};
use web_sys::MouseEvent;

use crate::{
    components::{
        app::AuthUser,
        files::uploader::UploaderMode,
        util::{empty_view::EmptyView, loading::Loading},
    },
    models::{
        order::Order,
        order_item::OrderItem,
        user::{Role, User},
        user_order::UserOrder,
    },
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
    let pool = crate::pool(cx)?;
    order.get_order_items(mode, &pool).await
}

#[server(DeleteOrderItemRequest, "/api")]
pub async fn delete_order_item_request(
    cx: Scope,
    order: Order,
    order_item: OrderItem,
) -> Result<UserOrder, ServerFnError> {
    let mode = order_item.mode;
    let pool = crate::pool(cx)?;
    let prefix = format!("/{:0>6}/{:?}", order.id, mode).to_lowercase();
    let path = format!("{prefix}/{}", order_item.file_name);
    crate::server::storage::delete_file(path).await?;
    OrderItem::delete(order_item.id, &pool).await?;
    order.revert_uploaded_status(mode, &pool).await?;
    UserOrder::get_by_order_id(order.id, &pool).await
}

#[server(RefreshGetUrlsRequest, "/api")]
pub async fn refresh_get_urls_request(
    cx: Scope,
    order_id: u64,
    mode: UploaderMode,
) -> Result<bool, ServerFnError> {
    use super::uploader::get_mime_type;
    use crate::server::storage::get_prefix;
    let (pool, _) = crate::server::pool_and_auth(cx)?;
    let order_items = OrderItem::get_order_items_by_order_id(order_id, mode, &pool).await?;
    for order_item in order_items {
        let file_name = order_item.file_name.clone();
        let Ok(mime_type) = get_mime_type(file_name.clone()) else {
                        return Err(ServerFnError::ServerError("Invalid file type".to_string()));
                    };
        let prefix = get_prefix(order_id, mode);
        let get_url =
            crate::server::storage::create_presigned_url(prefix, file_name, mime_type).await?;
        _ = order_item.update_get_url(get_url, &pool).await;
    }
    Ok(true)
}

#[component]
pub fn FileList(cx: Scope, order: Order, mode: UploaderMode) -> impl IntoView {
    let auth_user = use_context::<ReadSignal<AuthUser>>(cx).expect("Auth User should be present");
    let set_order = use_context::<WriteSignal<Option<UserOrder>>>(cx)
        .expect("Set Order Search should be present");
    let (to_delete, set_to_delete) = create_signal::<Option<OrderItem>>(cx, None);
    let delete_order_item_action = create_server_action::<DeleteOrderItemRequest>(cx);
    let refresh_get_urls_action = create_server_action::<RefreshGetUrlsRequest>(cx);
    let order_id = order.id;
    let order_d = order.clone();
    let get_order_items = create_resource(
        cx,
        move || {
            (
                delete_order_item_action.version().get(),
                refresh_get_urls_action.version().get(),
            )
        },
        move |_| {
            let order = order.clone();
            async move { get_order_items(cx, order, mode).await }
        },
    );
    let file_list_title = match mode {
        UploaderMode::Original => "Original Photos",
        UploaderMode::Processed => "Processed Photos",
    };
    let allow_delete = move || {
        let Some(User{role,..}) = auth_user.get() else {
            return false;
        };
        (mode == UploaderMode::Original && role == Role::Operator)
            || (mode == UploaderMode::Processed && role == Role::Processor)
    };
    let refresh_urls = move |_: MouseEvent| {
        refresh_get_urls_action.dispatch(RefreshGetUrlsRequest { order_id, mode });
    };
    let delete_dialog = create_node_ref::<Dialog>(cx);
    create_effect(cx, move |_| {
        let Some(Ok(order)) = delete_order_item_action.value().get() else {
            return;
        };
        set_order.set(Some(order));
    });
    view! { cx,
        <div class="container">
            <h2 class="header">{file_list_title}</h2>
            <Suspense fallback=move || {
                view! { cx, <Loading/> }
            }>
                <button on:click=refresh_urls>"Refresh Urls"</button>
                <div class="flex flex-wrap">
                    {move || match get_order_items.read(cx) {
                        None => {
                            view! { cx, <Loading/> }
                                .into_view(cx)
                        }
                        Some(order_items) => {
                            match order_items {
                                Err(e) => {
                                    view! { cx, <div>{e.to_string()}</div> }
                                        .into_view(cx)
                                }
                                Ok(order_items) => {
                                    if order_items.is_empty() {
                                        view! { cx, <div class="text-center w-full">"Not files uploaded"</div> }
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
                                                        <img src=&get_url/>
                                                    </div>
                                                    <div class="w-48 p-2 flex flex-col gap-y-5">
                                                        {if allow_delete() {
                                                            view! { cx,
                                                                <button class="red" on:click=delete_click>
                                                                    "Delete"
                                                                </button>
                                                            }
                                                                .into_view(cx)
                                                        } else {
                                                            view! { cx, <EmptyView/> }
                                                        }} <a class="button" href=&get_url>
                                                            "Download"
                                                        </a>
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
                                    None => {
                                        view! { cx, <EmptyView/> }
                                    }
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
                                            set_to_delete.set(None);
                                            delete_dialog.close();
                                        };
                                        view! { cx,
                                            <div class="text-lg">"Confirm delete"</div>
                                            <img src=get_url/>
                                            <button class="red" on:click=delete_click>
                                                "Delete"
                                            </button>
                                        }
                                            .into_view(cx)
                                    }
                                }
                            }
                        } <button on:click=move |_| {
                            let delete_dialog = delete_dialog.get().expect("Delete dialog should be present");
                            set_to_delete.set(None);
                            delete_dialog.close();
                        }>"Cancel"</button>
                    </div>
                </dialog>
            </Suspense>
        </div>
    }
}
