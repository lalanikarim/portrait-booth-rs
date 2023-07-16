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

#[server(StatusChangeRequest, "/api")]
pub async fn status_change_request(
    cx: Scope,
    order_id: u64,
    from: OrderStatus,
    to: OrderStatus,
) -> Result<UserOrder, ServerFnError> {
    match crate::server::pool_and_current_user(cx) {
        Err(e) => Err(e),
        Ok((pool, user)) => {
            if user.role != Role::Manager {
                Err(ServerFnError::ServerError(
                    "Only Manager is allowed to make this request".to_string(),
                ))
            } else {
                match crate::models::order::Order::update_status(order_id, from, to, &pool).await {
                    Err(e) => Err(e),
                    Ok(_) => UserOrder::get_by_order_id(order_id, &pool).await,
                }
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
        <MarkUploading order=order.clone() set_order/>
        <MarkUploaded order=order.clone() set_order/>
    }
    .into_view(cx)
}

#[component]
pub fn MarkUploaded(
    cx: Scope,
    order: UserOrder,
    set_order: WriteSignal<Option<UserOrder>>,
) -> impl IntoView {
    if order.status != OrderStatus::Uploading {
        return view! { cx, <EmptyView/> };
    }
    let status_change_action = create_server_action::<StatusChangeRequest>(cx);
    let change_status_conf = create_node_ref::<Dialog>(cx);
    let show_conf = move |_: MouseEvent| {
        let dialog = change_status_conf
            .get()
            .expect("Change Status Dialog should be present");
        _ = dialog.show_modal();
    };
    let close_conf = move |_: MouseEvent| {
        let dialog = change_status_conf
            .get()
            .expect("Change Status Dialog should be present");
        dialog.close();
    };
    let confirm_change = move |_: MouseEvent| {
        status_change_action.dispatch(StatusChangeRequest {
            order_id: order.id,
            from: order.status,
            to: OrderStatus::Uploaded,
        });
    };
    let disable_controls = move || status_change_action.pending().get();
    create_effect(cx, move |_| {
        if let Some(Ok(order)) = status_change_action.value().get() {
            let dialog = change_status_conf
                .get()
                .expect("Status Change Dialog should exist");
            dialog.close();
            set_order.set(Some(order));
        }
    });
    view! { cx,
        <button on:click=show_conf>"Set Uploaded Status"</button>
        <dialog _ref=change_status_conf>
            <h2>"Confirm order status change"</h2>
            <button on:click=confirm_change disabled=disable_controls>
                "Confirm"
            </button>
            <button on:click=close_conf class="red" disabled=disable_controls>
                "Cancel"
            </button>
        </dialog>
    }
    .into_view(cx)
}
#[component]
pub fn MarkUploading(
    cx: Scope,
    order: UserOrder,
    set_order: WriteSignal<Option<UserOrder>>,
) -> impl IntoView {
    if order.status != OrderStatus::InProcess && order.status != OrderStatus::Uploaded {
        return view! { cx, <EmptyView/> };
    }
    let status_change_action = create_server_action::<StatusChangeRequest>(cx);
    let change_status_conf = create_node_ref::<Dialog>(cx);
    let show_conf = move |_: MouseEvent| {
        let dialog = change_status_conf
            .get()
            .expect("Change Status Dialog should be present");
        _ = dialog.show_modal();
    };
    let close_conf = move |_: MouseEvent| {
        let dialog = change_status_conf
            .get()
            .expect("Change Status Dialog should be present");
        dialog.close();
    };
    let confirm_change = move |_: MouseEvent| {
        status_change_action.dispatch(StatusChangeRequest {
            order_id: order.id,
            from: order.status,
            to: OrderStatus::Uploading,
        });
    };
    let disable_controls = move || status_change_action.pending().get();
    create_effect(cx, move |_| {
        if let Some(Ok(order)) = status_change_action.value().get() {
            let dialog = change_status_conf
                .get()
                .expect("Status Change Dialog should exist");
            dialog.close();
            set_order.set(Some(order));
        }
    });
    view! { cx,
        <button on:click=show_conf>"Set Uploading Status"</button>
        <dialog _ref=change_status_conf>
            <h2>"Confirm order status change"</h2>
            <button on:click=confirm_change disabled=disable_controls>
                "Confirm"
            </button>
            <button on:click=close_conf class="red" disabled=disable_controls>
                "Cancel"
            </button>
        </dialog>
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
