use leptos::*;

use crate::{
    components::{
        orders::{create_order::CreateOrder, order_details::OrderDetails, order_list::OrderList},
        util::{empty_view::EmptyView, loading::Loading, show_error::ShowError},
    },
    models::user_order::UserOrder,
};

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::models::setting::Setting;
    }
}

#[server(GetAllowOrderCreationSetting, "/api")]
pub async fn get_allow_order_creation_setting(cx: Scope) -> Result<bool, ServerFnError> {
    match crate::pool(cx) {
        Err(e) => Err(e),
        Ok(pool) => Setting::get_allow_order_creation(&pool)
            .await
            .map(|setting| setting.is_true()),
    }
}

#[component]
pub fn OrdersView(cx: Scope) -> impl IntoView {
    let (order, set_order) = create_signal::<Option<UserOrder>>(cx, None);
    provide_context(cx, set_order);
    let order_creation_setting = create_resource(
        cx,
        move || order.get(),
        move |_| get_allow_order_creation_setting(cx),
    );

    view! { cx,
        <div>
            {move || match order.get() {
                Some(order) => {
                    view! { cx, <OrderDetails order/> }
                }
                None => {
                    view! { cx,
                        <OrderList/>
                        {move || {
                            match order_creation_setting.read(cx) {
                                None => view! { cx, <Loading/> },
                                Some(Err(e)) => view! { cx, <ShowError error=e.to_string()/> },
                                Some(Ok(true)) => view! { cx, <CreateOrder/> },
                                Some(Ok(false)) => view! { cx, <EmptyView/> },
                            }
                        }}
                    }
                        .into_view(cx)
                }
            }}
        </div>
    }
}
