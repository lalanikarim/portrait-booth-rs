use leptos::{html::Dialog, *};

use crate::models::order::{Order, OrderStatus};

#[server(DeleteOrderRequest, "/api")]
pub async fn delete_order_request(cx: Scope, order_id: u64) -> Result<bool, ServerFnError> {
    let pool = crate::pool(cx).expect("Pool should be present");
    let auth = crate::auth::auth(cx).expect("Auth should be present");
    let current_user = auth.current_user.expect("Logged in user should be present");
    Order::delete(order_id, current_user.id, &pool).await
}

#[server(StartCashPaymentRequest, "/api")]
pub async fn start_cash_payment_request(cx: Scope, order_id: u64) -> Result<bool, ServerFnError> {
    let pool = crate::pool(cx).expect("Pool should be present");
    let auth = crate::auth::auth(cx).expect("Auth should be present");
    let current_user = auth.current_user.expect("Logged in user should be present");
    Order::start_payment_cash(order_id, current_user.id, &pool).await
}

#[component]
pub fn OrderDetails(cx: Scope, order: Order) -> impl IntoView {
    let set_order = use_context::<WriteSignal<Option<Order>>>(cx)
        .expect("Set_order write signal should be present");
    let delete_order_action = create_server_action::<DeleteOrderRequest>(cx);
    let delete_conf_ref: NodeRef<Dialog> = create_node_ref(cx);
    let start_cash_payment_action = create_server_action::<StartCashPaymentRequest>(cx);
    let pay_cash_conf_ref: NodeRef<Dialog> = create_node_ref(cx);

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
    view! { cx,
        <div class="container">
            <h2 class="header">"Order Details"</h2>
            <dl>
                <dt>"Order #"</dt>
                <dd>{order.id}</dd>
                <dt>"No of Photos"</dt>
                <dd>{order.no_of_photos}</dd>
                <dt>"Order total"</dt>
                <dd>"$" {order.order_total}</dd>
            </dl>
            <button type="button" on:click=move |_| set_order.update(|o| *o = None)>
                "Back"
            </button>
            {move || {
                if order.status == OrderStatus::Created {
                    view! { cx,
                        <button
                            class="m-2"
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
                            class="red"
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
        </div>
    }
}
