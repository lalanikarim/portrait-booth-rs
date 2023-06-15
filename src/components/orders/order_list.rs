use leptos::*;

use crate::{
    components::orders::{create_order::CreateOrder, payment::Payment},
    models::order::Order,
};

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
    let get_orders_action = create_server_action::<GetOrdersRequest>(cx);
    get_orders_action.dispatch(GetOrdersRequest {});
    let orders_resource = create_resource(
        cx,
        move || order_created.version().get(),
        move |_| async move {
            let Ok(result) = get_orders_request(cx).await else {
            return Vec::new();
        };
            result
        },
    );
    let orders = move || orders_resource.read(cx).unwrap_or_default().into_iter();

    view! { cx,
        <div class="my-0 mx-auto max-w-sm text-center">
            <h2 class="p-6 text-4xl">"List of Orders"</h2>
            <CreateOrder order_created />
            <Suspense fallback=move || view!{cx, <div>"Loading..."</div>}>
            <table class="table-auto w-full">
                <thead>
                    <tr>
                        <th>"Order Id"</th>
                        <th>"No of Photos"</th>
                        <th>"Total"</th>
                        <th>"Status"</th>
                    </tr>
                    <For
                        each=orders
                        key=|order| order.id
                        view=move |cx, order| {
                            view! { cx, <OrderRow order=order.to_owned()/> }
                        }
                    />
                </thead>
            </table>
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
