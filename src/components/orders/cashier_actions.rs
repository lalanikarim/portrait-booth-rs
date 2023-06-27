use leptos::{html::Dialog, *};
use web_sys::MouseEvent;

use crate::{
    components::app::AuthUser,
    models::{
        order::{Order, OrderStatus, PaymentMode},
        user::Role,
        user_order::UserOrder,
    },
};

#[server(MarkPaidRequest, "/api")]
pub async fn mark_paid_request(cx: Scope, order_id: u64) -> Result<UserOrder, ServerFnError> {
    let pool = crate::pool(cx).expect("Pool should be present");
    let auth = crate::auth::auth(cx).expect("Auth should be present");
    let Some(user) = auth.current_user else {
        return Err(ServerFnError::ServerError("Unable to fetch logged in user".to_string()));
    };
    if user.role != Role::Cashier && user.role != Role::Manager {
        return Err(ServerFnError::ServerError(
            "Only Manager or Cashier can collect payment".to_string(),
        ));
    }
    match Order::collect_payment_cash(order_id, user.id, &pool).await {
        Err(e) => Err(e),
        Ok(false) => Err(ServerFnError::ServerError(
            "Unable to save changes to order".to_string(),
        )),
        Ok(true) => UserOrder::get_by_order_id(order_id, &pool).await,
    }
}

#[component]
pub fn CashierActions(cx: Scope, order: UserOrder) -> impl IntoView {
    let auth_user = use_context::<AuthUser>(cx).expect("AuthUser should exist");
    if let Some(user) = auth_user.get() {
        if user.role != Role::Cashier {
            return view! {cx,<div />}.into_view(cx);
        }
    }
    if order.status != OrderStatus::PaymentPending || order.mode_of_payment != PaymentMode::Cash {
        return view! {cx,<div />}.into_view(cx);
    }
    let mark_paid_action = create_server_action::<MarkPaidRequest>(cx);
    let disable_controls = move || mark_paid_action.pending().get();
    let set_order = use_context::<WriteSignal<Option<UserOrder>>>(cx)
        .expect("Set_order write signal should be present");
    let cashier_conf = create_node_ref::<Dialog>(cx);
    let mark_paid_click = move |_: MouseEvent| {
        let dialog = cashier_conf.get().expect("Mark Paid Dialog should exist");
        _ = dialog.show_modal();
    };
    let confirm_click = move |_: MouseEvent| {
        mark_paid_action.dispatch(MarkPaidRequest { order_id: order.id });
    };
    let cancel_click = move |_: MouseEvent| {
        let dialog = cashier_conf.get().expect("Mark Paid Dialog should exist");
        dialog.close();
    };
    create_effect(cx, move |_| {
        if let Some(Ok(order)) = mark_paid_action.value().get() {
            let dialog = cashier_conf.get().expect("Mark Paid Dialog should exist");
            dialog.close();
            set_order.set(Some(order));
        }
    });
    view! { cx,
        <button on:click=mark_paid_click>"Mark Paid"</button>
        <dialog _ref=cashier_conf>
            <h2>"Confirm payment for " {order.name} " total $" {order.order_total}</h2>
            <button disabled=disable_controls on:click=confirm_click>"Confirm"</button>
            <button disabled=disable_controls on:click=cancel_click class="red">"Cancel"</button>
        </dialog>
    }
    .into_view(cx)
}
