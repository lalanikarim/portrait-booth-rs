use leptos::*;

use crate::{
    components::{
        files::{
            file_list::FileList,
            uploader::{Uploader, UploaderMode},
        },
        util::{empty_view::EmptyView, loading::Loading, show_error::ShowError},
    },
    models::{
        order::{Order, OrderStatus},
        user_order::UserOrder,
    },
};

#[server(FetchOrderRequest, "/api")]
pub async fn fetch_order_request(cx: Scope) -> Result<Option<Order>, ServerFnError> {
    match crate::server::pool_and_current_user(cx) {
        Err(e) => Err(e),
        Ok((
            pool,
            crate::models::user::User {
                id: processor_id,
                role: crate::models::user::Role::Processor,
                ..
            },
        )) => Order::fetch_order_for_processor(processor_id, &pool).await,
        Ok(_) => Err(ServerFnError::ServerError(
            "Only processors are allowed to make this request".to_string(),
        )),
    }
}

#[server(MarkReadyForDeliveryRequest, "/api")]
pub async fn mark_ready_for_delivery_request(cx: Scope) -> Result<bool, ServerFnError> {
    match fetch_order_request(cx).await {
        Err(e) => Err(e),
        Ok(None) => Err(ServerFnError::Args("Invalid order".to_string())),
        Ok(Some(order)) => match crate::pool(cx) {
            Ok(pool) => order.mark_order_ready_for_delivery(&pool).await,
            Err(e) => Err(e),
        },
    }
}

#[component]
pub fn ProcessorView(cx: Scope) -> impl IntoView {
    let (user_order, set_user_order) = create_signal::<Option<UserOrder>>(cx, None);
    _ = provide_context(cx, set_user_order);
    let order_resource = create_resource(
        cx,
        move || user_order.get(),
        move |_| async move { fetch_order_request(cx).await },
    );
    view! { cx,
        <div class="container">
            <h2 class="header">"Process Orders"</h2>
        </div>
        {move || {
            match order_resource.read(cx) {
                None => view! { cx, <Loading/> },
                Some(Err(e)) => view! { cx, <ShowError error=e.to_string()/> },
                Some(Ok(None)) => view! { cx, <EmptyView/> },
                Some(Ok(Some(order))) => {
                    view! { cx,
                        <FileList order=order.clone() mode=UploaderMode::Original/>
                        <FileList order=order.clone() mode=UploaderMode::Processed/>
                        {move || {
                            match order.clone().status {
                                OrderStatus::InProcess => {
                                    view! { cx,
                                        <Uploader
                                            order=order.clone()
                                            mode=UploaderMode::Processed
                                            order_resource
                                        />
                                    }
                                        .into_view(cx)
                                }
                                OrderStatus::Processed => {
                                    view! { cx, <MarkReadyForDelivery /> }
                                }
                                _ => view! { cx, <EmptyView/> },
                            }
                        }}
                    }
                        .into_view(cx)
                }
            }
        }}
    }
}

#[component]
pub fn MarkReadyForDelivery(cx: Scope) -> impl IntoView {
    let set_order = use_context::<WriteSignal<Option<UserOrder>>>(cx)
        .expect("Set Order Search should be present");
    let mark_complete_action = create_server_action::<MarkReadyForDeliveryRequest>(cx);
    create_effect(cx, move |_| {
        if let Some(Ok(true)) = mark_complete_action.value().get() {
            set_order.set(None);
        };
    });
    view! { cx,
        <div class="container">
            <h2 class="header">"All Photos Uploaded"</h2>
            <button on:click=move|_|{mark_complete_action.dispatch(MarkReadyForDeliveryRequest{})}>"Complete Order"</button>
        </div>
    }
    .into_view(cx)
}
