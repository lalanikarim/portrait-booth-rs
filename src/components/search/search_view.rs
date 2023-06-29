use leptos::*;

use crate::{
    components::{
        app::AuthUser,
        files::{
            file_list::FileList,
            uploader::{Uploader, UploaderMode},
        },
        orders::order_details::OrderDetails,
        search::{order_search::OrderSearch, search_results::SearchResults},
        util::empty_view::EmptyView,
        util::loading::Loading,
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

#[server(OrderSearchRequest, "/api")]
pub async fn order_search_request(
    cx: Scope,
    form: crate::models::user_order::OrderSearchForm,
) -> Result<Vec<UserOrder>, ServerFnError> {
    let pool = crate::pool(cx).expect("Pool should exist");
    UserOrder::search_orders(form, &pool).await
}
#[component]
pub fn SearchView(cx: Scope) -> impl IntoView {
    let (order, set_order) = create_signal::<Option<UserOrder>>(cx, None);
    let auth_user = use_context::<ReadSignal<AuthUser>>(cx).expect("Auth User should be present");

    let order_search_action = create_server_action::<OrderSearchRequest>(cx);
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
    view! { cx,
        <OrderSearch order_search_action/>
        {move || match order.get() {
            None => {
                view! { cx, <SearchResults orders=order_search_action.value()/> }
                    .into_view(cx)
            }
            Some(order) => {
                let mut views = Vec::new();
                views
                    .push(
                        view! { cx, <OrderDetails order=order.clone()/> },
                    );
                if let Some(user) = auth_user.get() {
                    if user.role == Role::Operator && order.status == OrderStatus::Paid {
                        views
                            .push(
                                view! { cx,
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
                                },
                            );
                    }
                }
                views.collect_view(cx)
            }
        }}
    }
}
