use leptos::{html::Dialog, *};
use web_sys::MouseEvent;

use crate::{
    components::{app::AuthUser, util::empty_view::EmptyView},
    models::{
        order::{OrderStatus, PaymentMode},
        user::Role,
        user_order::UserOrder,
    },
};

#[server(MarkStripePaidRequest, "/api")]
pub async fn mark_stripe_paid_request(
    cx: Scope,
    order_id: u64,
) -> Result<UserOrder, ServerFnError> {
    match crate::server::pool_and_current_user(cx) {
        Err(e) => Err(e),
        Ok((pool, user)) => {
            if user.role != Role::Manager {
                return Err(ServerFnError::ServerError(
                    "Only Manager can mark pending stripe payment to paid".to_string(),
                ));
            }
            let manager_override = format!("Manager override by {},{}", user.name, user.email);
            match crate::models::order::Order::get_by_id(order_id, &pool).await {
                Err(e) => Err(e),
                Ok(None) => Err(ServerFnError::Args("Invalid Order Id".to_string())),
                Ok(Some(order)) => match order
                    .mark_stripe_payment_complete(manager_override, &pool)
                    .await
                {
                    Err(e) => Err(e),
                    Ok(false) => Err(ServerFnError::ServerError(
                        "Unable to save changes to order".to_string(),
                    )),
                    Ok(true) => UserOrder::get_by_order_id(order_id, &pool).await,
                },
            }
        }
    }
}

#[server(ClearStripePendingStatus, "/api")]
pub async fn clear_stripe_pending_status(
    cx: Scope,
    order_id: u64,
) -> Result<UserOrder, ServerFnError> {
    match crate::server::pool_and_current_user(cx) {
        Err(e) => Err(e),
        Ok((pool, user)) => {
            if user.role != Role::Manager {
                return Err(ServerFnError::ServerError(
                    "Only Manager can reset pending payment status".to_string(),
                ));
            }
            match crate::models::order::Order::get_by_id(order_id, &pool).await {
                Err(e) => Err(e),
                Ok(None) => Err(ServerFnError::Args("Invalid Order Id provided".to_string())),
                Ok(Some(order)) => match order.reset_payment_status(&pool).await {
                    Err(e) => Err(e),
                    Ok(None) => Err(ServerFnError::ServerError(
                        "Unable to retrieve result".to_string(),
                    )),
                    Ok(Some(order)) => UserOrder::get_by_order_id(order.id, &pool).await,
                },
            }
        }
    }
}

#[component]
pub fn ManagerActions(cx: Scope, order: UserOrder) -> impl IntoView {
    let auth_user = use_context::<ReadSignal<AuthUser>>(cx).expect("AuthUser should exist");
    if let Some(user) = auth_user.get() {
        if user.role != Role::Manager {
            return view! { cx, <EmptyView/> }.into_view(cx);
        }
    }
    let set_order = use_context::<WriteSignal<Option<UserOrder>>>(cx)
        .expect("Set_order write signal should be present");
    let mark_paid_action = create_server_action::<MarkStripePaidRequest>(cx);
    let clear_status_action = create_server_action::<ClearStripePendingStatus>(cx);
    view! { cx,
        <ResetPendingPayment order=order.clone() set_order clear_status_action/>
        <MarkStripePaid order=order.clone() set_order mark_paid_action/>
    }
    .into_view(cx)
}

#[component]
pub fn ResetPendingPayment(
    cx: Scope,
    order: UserOrder,
    set_order: WriteSignal<Option<UserOrder>>,
    clear_status_action: Action<ClearStripePendingStatus, Result<UserOrder, ServerFnError>>,
) -> impl IntoView {
    if order.status != OrderStatus::PaymentPending {
        return view! { cx, <EmptyView/> };
    }
    let clear_conf = create_node_ref::<Dialog>(cx);
    let clear_status_click = move |_: MouseEvent| {
        let dialog = clear_conf.get().expect("Clear Status Dialog should exist");
        _ = dialog.show_modal();
    };

    let clear_cancel_click = move |_: MouseEvent| {
        let dialog = clear_conf.get().expect("Mark Paid Dialog should exist");
        dialog.close();
    };
    let clear_confirm_click = move |_: MouseEvent| {
        clear_status_action.dispatch(ClearStripePendingStatus { order_id: order.id });
    };
    let disable_clear_controls = move || clear_status_action.pending().get();
    create_effect(cx, move |_| {
        if let Some(Ok(order)) = clear_status_action.value().get() {
            let dialog = clear_conf.get().expect("Mark Paid Dialog should exist");
            dialog.close();
            set_order.set(Some(order));
        }
    });
    view! { cx,
        <button on:click=clear_status_click>"Clear Pending Status"</button>
        <dialog _ref=clear_conf>
            <h2>"Confirm clear pending status for " {order.name} " total $" {order.order_total}</h2>
            <button disabled=disable_clear_controls on:click=clear_confirm_click>
                "Confirm"
            </button>
            <button disabled=disable_clear_controls on:click=clear_cancel_click>
                "Cancel"
            </button>
        </dialog>
    }
    .into_view(cx)
}

#[component]
pub fn MarkStripePaid(
    cx: Scope,
    order: UserOrder,
    set_order: WriteSignal<Option<UserOrder>>,
    mark_paid_action: Action<MarkStripePaidRequest, Result<UserOrder, ServerFnError>>,
) -> impl IntoView {
    if order.status != OrderStatus::PaymentPending || order.mode_of_payment != PaymentMode::Stripe {
        return view! { cx, <EmptyView/> };
    }
    let manager_conf = create_node_ref::<Dialog>(cx);
    let disable_controls = move || mark_paid_action.pending().get();
    let confirm_click = move |_: MouseEvent| {
        mark_paid_action.dispatch(MarkStripePaidRequest { order_id: order.id });
    };
    let cancel_click = move |_: MouseEvent| {
        let dialog = manager_conf.get().expect("Mark Paid Dialog should exist");
        dialog.close();
    };
    let mark_paid_click = move |_: MouseEvent| {
        let dialog = manager_conf.get().expect("Mark Paid Dialog should exist");
        _ = dialog.show_modal();
    };
    create_effect(cx, move |_| {
        if let Some(Ok(order)) = mark_paid_action.value().get() {
            let dialog = manager_conf.get().expect("Mark Paid Dialog should exist");
            dialog.close();
            set_order.set(Some(order));
        }
    });
    view! { cx,
        <button on:click=mark_paid_click>"Mark Stripe Paid"</button>
        <dialog _ref=manager_conf>
            <h2>
                "Confirm stripe payment for " {order.name.clone()} " total $" {order.order_total}
            </h2>
            <button disabled=disable_controls on:click=confirm_click>
                "Confirm"
            </button>
            <button disabled=disable_controls on:click=cancel_click class="red">
                "Cancel"
            </button>
        </dialog>
    }
    .into_view(cx)
}
