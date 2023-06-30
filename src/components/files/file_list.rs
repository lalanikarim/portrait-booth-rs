use leptos::*;

use crate::{
    components::{files::uploader::UploaderMode, util::loading::Loading},
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

#[component]
pub fn FileList(cx: Scope, order: Order, mode: UploaderMode) -> impl IntoView {
    let get_order_items = create_resource(
        cx,
        || (),
        move |_| {
            let order = order.clone();
            async move { get_order_items(cx, order, mode).await }
        },
    );
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
                                            .iter()
                                            .map(|order_item| {
                                                let get_url = order_item.clone().get_url.clone();
                                                view! { cx,
                                                    <div class="w-48 p-2">
                                                        <a href=&get_url>
                                                            <img src=&get_url/>
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
            </Suspense>
        </div>
    }
}
