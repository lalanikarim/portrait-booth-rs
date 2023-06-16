use leptos::*;

use crate::{components::orders::create_order::CreateOrder, models::order::Order};

#[server(GetOrdersRequest, "/api")]
pub async fn get_orders_request(cx: Scope) -> Result<Vec<Order>, ServerFnError> {
    let pool = crate::pool(cx).expect("Pool should exist");
    let auth = crate::auth::auth(cx).expect("Auth should exist");
    let current_user = auth.current_user.expect("Authenticated User should exist");
    current_user.orders(&pool).await
}

#[component]
pub fn OrderList(cx: Scope) -> impl IntoView {
    let order_created = create_action(cx, |()| async {});
    let orders_resource = create_resource(
        cx,
        move || order_created.version().get(),
        move |_| async move { get_orders_request(cx).await },
    );

    view! { cx,
        <CreateOrder order_created/>
        <div class="container">
            <h2 class="header">"List of Orders"</h2>
            <Suspense fallback=move || {
                view! { cx, <div>"Loading..."</div> }
            }>
                {move || {
                    orders_resource
                        .read(cx)
                        .map(|orders| match orders {
                            Err(_) => {
                                view! { cx, <div class="error">"Error fetching orders"</div> }
                                    .into_view(cx)
                            }
                            Ok(orders) => {
                                if orders.len() > 0 {
                                    view! { cx,
                                        <table class="table-auto w-full">
                                            <thead>
                                                <tr>
                                                    <th>"Order Id"</th>
                                                    <th>"No of Photos"</th>
                                                    <th>"Total"</th>
                                                    <th>"Status"</th>
                                                </tr>
                                                <For
                                                    each=move || orders.clone()
                                                    key=|order| order.id
                                                    view=move |cx, order| {
                                                        view! { cx, <OrderRow order=order.to_owned()/> }
                                                    }
                                                />
                                            </thead>
                                        </table>
                                    }
                                        .into_view(cx)
                                } else {
                                    view! { cx, <div>"No orders found"</div> }
                                        .into_view(cx)
                                }
                            }
                        })
                }}
            </Suspense>
        </div>
    }
}

#[component]
pub fn OrderRow(cx: Scope, order: Order) -> impl IntoView {
    view! { cx,
        <tr>
            <td>{order.id}</td>
            <td>{order.no_of_photos}</td>
            <td>"$" {order.order_total}</td>
            <td>{format!("{:?}", order.status)}</td>
        </tr>
    }
}
