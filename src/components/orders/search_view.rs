use leptos::*;

use crate::{
    components::{
        app::AuthUser,
        empty_view::EmptyView,
        files::{
            file_list::FileList,
            uploader::{Uploader, UploaderMode},
        },
        loading::Loading,
        orders::{order_details::OrderDetails, order_search::OrderSearch},
    },
    models::{
        order::{Order, OrderStatus},
        user::Role,
        user_order::UserOrder,
    },
};

#[server(GetOrderRequest, "/api")]
pub async fn get_order_request(cx: Scope, id: u64) -> Result<Option<Order>, ServerFnError> {
    let pool = crate::pool(cx).expect("Pool should be present");
    Order::get_by_id(id, &pool).await
}

#[component]
pub fn SearchView(cx: Scope) -> impl IntoView {
    let (order, set_order) = create_signal::<Option<UserOrder>>(cx, None);
    let auth_user = use_context::<ReadSignal<AuthUser>>(cx).expect("Auth User should be present");

    let order_resource = create_resource(
        cx,
        move || order.get(),
        move |order| async move {
            match order {
                None => Ok(None),
                Some(UserOrder { id, .. }) => get_order_request(cx, id).await,
            }
        },
    );
    provide_context(cx, set_order);

    move || match order.get() {
        None => view! { cx, <OrderSearch/> }.into_view(cx),
        Some(order) => {
            let mut views = Vec::new();
            views.push(view! { cx, <OrderDetails order=order.clone()/> });
            if let Some(user) = auth_user.get() {
                if user.role == Role::Operator && order.status == OrderStatus::Paid {
                    views.push(view! { cx,
                        <Suspense fallback=move || {
                            view! { cx, <Loading/> }
                        }>
                            {move || {
                                match order_resource.read(cx) {
                                    None => {
                                        view! { cx, <Loading/> }
                                    }
                                    Some(resource) => {
                                        match resource {
                                            Ok(Some(order)) => {
                                                let mode = UploaderMode::Original;
                                                view! { cx,
                                                    <FileList order=order.clone() mode/>
                                                    <Uploader order=order.clone() mode/>
                                                }
                                                    .into_view(cx)
                                            }
                                            Ok(None) => {
                                                view! { cx, <EmptyView/> }
                                            }
                                            Err(e) => {
                                                view! { cx, <div class="red">{e.to_string()}</div> }
                                                    .into_view(cx)
                                            }
                                        }
                                    }
                                }
                            }}
                        </Suspense>
                    });
                }
            }
            views.collect_view(cx)
        }
    }
}
