use leptos::*;

use crate::{
    components::orders::{order_details::OrderDetails, order_list::OrderList, UnitPrice},
    models::order::Order,
};
#[server(GetUnitPrice, "/api")]
pub async fn get_unit_price(cx: Scope) -> Result<u64, ServerFnError> {
    dotenv!("PHOTO_UNIT_PRICE")
        .parse::<u64>()
        .map_err(|e| crate::to_server_fn_error(e))
}
#[component]
pub fn OrdersView(cx: Scope) -> impl IntoView {
    let (order, set_order) = create_signal::<Option<Order>>(cx, None);
    let unit_price_resource = create_resource(
        cx,
        || (),
        move |_| async move { get_unit_price(cx).await.map(|p| UnitPrice(p)) },
    );
    provide_context(cx, set_order);
    provide_context(cx, unit_price_resource);

    view! { cx,
        <div>
            {move || match order.get() {
                Some(order) => {
                    view! { cx, <OrderDetails order/> }
                }
                None => {
                    view! { cx, <OrderList/> }
                }
            }}
        </div>
    }
}
