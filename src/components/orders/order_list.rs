use leptos::{ev::MouseEvent, *};

use crate::{
    components::util::loading::Loading,
    models::{order::OrderStatus, user_order::UserOrder},
};

#[server(GetOrdersRequest, "/api")]
pub async fn get_orders_request(cx: Scope) -> Result<Vec<UserOrder>, ServerFnError> {
    match crate::server::pool_and_current_user(cx) {
        Err(e) => Err(e),
        Ok((pool, current_user)) => current_user.orders(&pool).await,
    }
}

#[component]
pub fn OrderList(cx: Scope) -> impl IntoView {
    let refresh_list_action = create_action(cx, |()| async {});
    let orders_resource = create_resource(
        cx,
        move || refresh_list_action.version().get(),
        move |_| async move { get_orders_request(cx).await },
    );

    view! { cx,
        <div class="container">
            <h2 class="header">"List of Orders"</h2>
            <div class="pb-5">
                <button on:click=move |_| refresh_list_action.dispatch(())>"Refresh"</button>
            </div>
            {move || {
                match orders_resource.read(cx) {
                    None => {
                        view! { cx, <Loading/> }
                    }
                    Some(orders) => {
                        match orders {
                            Err(_) => {
                                view! { cx, <div class="error">"Error fetching orders"</div> }
                                    .into_view(cx)
                            }
                            Ok(orders) => {
                                if orders.is_empty() {
                                    view! { cx, <div>"No orders found"</div> }
                                        .into_view(cx)
                                } else {
                                    view! { cx,
                                        <table class="table-auto w-full broder-collapse border border-slate-400">
                                            <thead class="bg-slate-50">
                                                <tr>
                                                    <th class="border border-slate-300 p-1 w-1/5">"#"</th>
                                                    <th class="border border-slate-300 p-1 w-1/5">
                                                        "No of Pics"
                                                    </th>
                                                    <th class="border border-slate-300 p-1 w-1/5">"Total"</th>
                                                    <th class="border border-slate-300 p-1 w-2/5">"Status"</th>
                                                </tr>
                                            </thead>
                                            <tbody>
                                                {orders
                                                    .into_iter()
                                                    .map(move |order| {
                                                        view! { cx, <OrderRow order/> }
                                                    })
                                                    .collect_view(cx)}
                                            </tbody>
                                        </table>
                                    }
                                        .into_view(cx)
                                }
                            }
                        }
                    }
                }
            }}
        </div>
    }
}

#[component]
pub fn OrderRow(cx: Scope, order: UserOrder) -> impl IntoView {
    let set_order = use_context::<WriteSignal<Option<UserOrder>>>(cx)
        .expect("Set_order write signal should be present");
    let o = order.clone();
    let on_click = move |_ev: MouseEvent| {
        let order = order.clone();
        set_order.update(|o| *o = Some(order));
    };
    let status = move || match o.status {
        OrderStatus::PaymentPending => format!("{:?} ({:?})", o.status, o.mode_of_payment),
        _ => format!("{:?}", o.status),
    };
    view! { cx,
        <tr>
            <td class="border border-slate-300 p-1">{o.id}</td>
            <td class="border border-slate-300 p-1">{o.no_of_photos}</td>
            <td class="border border-slate-300 p-1">"$" {o.order_total}</td>
            <td class="border border-slate-300 p-1" on:click=on_click>
                {status}
            </td>
        </tr>
    }
}
