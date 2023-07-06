use leptos::*;

use crate::{
    components::orders::{
        create_order::CreateOrder, order_details::OrderDetails, order_list::OrderList,
    },
    models::user_order::UserOrder,
};

#[component]
pub fn OrdersView(cx: Scope) -> impl IntoView {
    let (order, set_order) = create_signal::<Option<UserOrder>>(cx, None);
    provide_context(cx, set_order);

    view! { cx,
        <div>
            {move || match order.get() {
                Some(order) => {
                    view! { cx, <OrderDetails order/> }
                }
                None => {
                    view! { cx,
                        <CreateOrder />
                        //<OrderList />
                    }
                        .into_view(cx)
                }
            }}
        </div>
    }
}
