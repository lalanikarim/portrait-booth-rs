use leptos::*;

use crate::{
    components::orders::{order_details::OrderDetails, order_search::OrderSearch},
    models::user_order::UserOrder,
};

#[component]
pub fn SearchView(cx: Scope) -> impl IntoView {
    let (order, set_order) = create_signal::<Option<UserOrder>>(cx, None);
    provide_context(cx, set_order);

    move || match order.get() {
        None => view! {cx,<OrderSearch />}.into_view(cx),
        Some(order) => view! {cx,<OrderDetails order />}.into_view(cx),
    }
}
