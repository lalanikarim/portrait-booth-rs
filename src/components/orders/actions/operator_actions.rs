use crate::components::app::AuthUser;
use crate::components::util::empty_view::EmptyView;
use crate::models::order::OrderStatus;
use crate::models::user::Role;
use crate::models::user_order::UserOrder;
use leptos::*;
use web_sys::MouseEvent;

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::models::order::Order;
    }
}

#[server(OrderStatusChangeRequest, "/api")]
pub async fn order_status_change_request(
    cx: Scope,
    order: UserOrder,
    from: OrderStatus,
    to: OrderStatus,
) -> Result<UserOrder, ServerFnError> {
    let pool = crate::pool(cx)?;
    let success = Order::update_status(order.id, from, to, &pool).await?;
    if !success {
        return Ok(order);
    }
    UserOrder::get_by_order_id(order.id, &pool).await
}

#[component]
pub fn OperatorActions(cx: Scope, order: UserOrder) -> impl IntoView {
    let auth_user = use_context::<ReadSignal<AuthUser>>(cx).expect("AuthUser should exist");
    let set_order =
        use_context::<WriteSignal<Option<UserOrder>>>(cx).expect("Set Order should be present");
    let order_status_change_action = create_server_action::<OrderStatusChangeRequest>(cx);
    if let Some(user) = auth_user.get() {
        if user.role != Role::Operator {
            return view! {cx,<EmptyView/>}.into_view(cx);
        }
    }
    let status = order.status;
    let start_uploading_order = order.clone();
    let start_uploading = move |_: MouseEvent| {
        let order = start_uploading_order.clone();
        let from = OrderStatus::Paid;
        let to = OrderStatus::Uploading;
        order_status_change_action.dispatch(OrderStatusChangeRequest { order, from, to });
    };

    create_effect(cx, move |_| {
        if let Some(Ok(order)) = order_status_change_action.value().get() {
            set_order.set(Some(order));
        }
    });
    match status {
        OrderStatus::Paid => {
            view! {cx,<button on:click=start_uploading>"Start Uploading"</button>}.into_view(cx)
        }
        _ => view! {cx, <EmptyView/>},
    }
}
