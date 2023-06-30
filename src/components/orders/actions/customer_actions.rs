use crate::{
    components::app::AuthUser,
    models::{order::OrderStatus, user_order::UserOrder},
};
use leptos::{html::Dialog, *};

#[cfg(feature = "ssr")]
use crate::models::order::Order;

#[server(DeleteOrderRequest, "/api")]
pub async fn delete_order_request(cx: Scope, order_id: u64) -> Result<bool, ServerFnError> {
    match crate::server::pool_and_current_user(cx) {
        Err(e) => Err(e),
        Ok((pool, crate::models::user::User { id, .. })) => {
            Order::delete(order_id, id, &pool).await
        }
    }
}

#[server(StartCashPaymentRequest, "/api")]
pub async fn start_cash_payment_request(cx: Scope, order_id: u64) -> Result<bool, ServerFnError> {
    match crate::server::pool_and_current_user(cx) {
        Err(e) => Err(e),
        Ok((pool, crate::models::user::User { id, .. })) => {
            Order::start_payment_cash(order_id, id, &pool).await
        }
    }
}

#[server(StartStripePaymentRequest, "/api")]
pub async fn start_stripe_payment_request(
    cx: Scope,
    order_id: u64,
) -> Result<String, ServerFnError> {
    use crate::server::stripe::get_payment_link;
    match crate::server::pool_and_current_user(cx) {
        Err(e) => Err(e),
        Ok((pool, current_user)) => {
            let order_ref = format!("Email: {}, Order #:{}", current_user.email, order_id);

            match Order::start_payment_stripe(order_id, current_user.id, order_ref, &pool).await {
                Ok(true) => match Order::get_by_id(order_id, &pool).await {
                    Err(e) => Err(e),
                    Ok(None) => Err(ServerFnError::ServerError(
                        "Unable to fetch order".to_string(),
                    )),
                    Ok(Some(order)) => get_payment_link(cx, order).await,
                },
                Ok(false) => Err(ServerFnError::ServerError(
                    "Error starting Stripe Request".to_string(),
                )),
                Err(e) => Err(e),
            }
        }
    }
}
#[component]
pub fn CustomerActions(cx: Scope, order: UserOrder) -> impl IntoView {
    let auth_user = use_context::<ReadSignal<AuthUser>>(cx).expect("AuthUser should be present");
    if let Some(user) = auth_user.get() {
        if user.id != order.customer_id {
            return view! {cx,<div />}.into_view(cx);
        }
    }
    let delete_order_action = create_server_action::<DeleteOrderRequest>(cx);
    let delete_conf_ref: NodeRef<Dialog> = create_node_ref(cx);
    let start_cash_payment_action = create_server_action::<StartCashPaymentRequest>(cx);
    let pay_cash_conf_ref: NodeRef<Dialog> = create_node_ref(cx);
    let start_stripe_payment_action = create_server_action::<StartStripePaymentRequest>(cx);
    let pay_stripe_conf_ref: NodeRef<Dialog> = create_node_ref(cx);
    let set_order = use_context::<WriteSignal<Option<UserOrder>>>(cx)
        .expect("Set_order write signal should be present");
    create_effect(cx, move |_| {
        if let Some(Ok(true)) = delete_order_action.value().get() {
            let dialog = delete_conf_ref.get().expect("dialog should be present");
            dialog.close();
            set_order.update(|o| *o = None);
        }
    });
    create_effect(cx, move |_| {
        if let Some(Ok(true)) = start_cash_payment_action.value().get() {
            let dialog = pay_cash_conf_ref.get().expect("dialog should be present");
            dialog.close();
            set_order.update(|o| *o = None);
        }
    });
    create_effect(cx, move |_| {
        if let Some(Ok(url)) = start_stripe_payment_action.value().get() {
            let window = leptos::window();
            _ = window.location().set_href(&url);
        }
    });
    view! { cx,
        {move || {
            if order.status == OrderStatus::Created {
                view! { cx,
                    <button
                        class="m-1"
                        type="button"
                        on:click=move |ev| {
                            ev.prevent_default();
                            let dialog = pay_stripe_conf_ref.get().expect("dialog should be present");
                            _ = dialog.show_modal();
                            start_stripe_payment_action
                                .dispatch(StartStripePaymentRequest {
                                    order_id: order.id,
                                });
                        }
                    >
                        "Pay with Stripe"
                    </button>
                    <dialog node_ref=pay_stripe_conf_ref>
                        <h2>"Redirecting to Stripe. Please wait..."</h2>
                    </dialog>
                    <button
                        class="m-1"
                        type="button"
                        on:click=move |ev| {
                            ev.prevent_default();
                            let dialog = pay_cash_conf_ref.get().expect("dialog should be present");
                            _ = dialog.show_modal();
                        }
                    >
                        "Pay with Cash"
                    </button>
                    <dialog node_ref=pay_cash_conf_ref>
                        <h2>"Please pay the cashier"</h2>
                        <button on:click=move |_| {
                            start_cash_payment_action
                                .dispatch(StartCashPaymentRequest {
                                    order_id: order.id,
                                });
                        }>"Close"</button>
                    </dialog>
                    <button
                        type="button"
                        class="red m-1"
                        on:click=move |ev| {
                            ev.prevent_default();
                            let dialog = delete_conf_ref.get().expect("dialog should be present");
                            _ = dialog.show_modal();
                        }
                    >
                        "Delete"
                    </button>
                    <dialog node_ref=delete_conf_ref>
                        <h2>"Are you sure you want to delete this order?"</h2>
                        <button on:click=move |_| {
                            delete_order_action
                                .dispatch(DeleteOrderRequest {
                                    order_id: order.id,
                                });
                        }>"Close"</button>
                    </dialog>
                }
                    .into_view(cx)
            } else {
                view! { cx, <div></div> }
                    .into_view(cx)
            }
        }}
    }.into_view(cx)
}
