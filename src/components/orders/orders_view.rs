use leptos::*;

use crate::{
    components::orders::{order_details::OrderDetails, order_list::OrderList},
    models::order::Order,
};

#[component]
pub fn OrdersView(cx: Scope) -> impl IntoView {
    let (order, set_order) = create_signal::<Option<Order>>(cx, None);
    provide_context(cx, set_order);
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
