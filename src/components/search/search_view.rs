use leptos::*;

use crate::{
    components::{
        app::AuthUser,
        files::{file_list::FileList, uploader::UploaderMode},
        orders::order_details::OrderDetails,
        search::{
            operator_uploader::OperatorUploader, order_search::OrderSearch,
            search_results::SearchResults,
        },
    },
    models::{
        order::{Order, OrderStatus},
        user::Role,
        user_order::UserOrder,
    },
};

#[server(GetOrderRequest, "/api")]
pub async fn get_order_request(cx: Scope, id: u64) -> Result<Option<Order>, ServerFnError> {
    let pool = crate::pool(cx)?;
    Order::get_by_id(id, &pool).await
}

#[server(OrderSearchRequest, "/api")]
pub async fn order_search_request(
    cx: Scope,
    form: crate::models::user_order::OrderSearchForm,
) -> Result<Vec<UserOrder>, ServerFnError> {
    let pool = crate::pool(cx)?;
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
            None => view! { cx, <SearchResults orders=order_search_action.value()/> }.into_view(cx),
            Some(order) => {
                let mut views = Vec::new();
                views.push(view! { cx, <OrderDetails order=order.clone()/> });
                if let Some(user) = auth_user.get() {
                    if user.role == Role::Operator
                        && (order.status == OrderStatus::Uploading
                            || order.status == OrderStatus::Uploaded)
                    {
                        views.push(view! { cx, <OperatorUploader order_resource/> });
                    }
                    if user.role == Role::Manager {
                        if let Some(Ok(Some(order))) = order_resource.read(cx) {
                            views
                                .push(
                                    view! { cx,
                                        <FileList order=order.clone() mode=UploaderMode::Original/>
                                        <FileList order=order.clone() mode=UploaderMode::Processed/>
                                    }
                                        .into_view(cx),
                                );
                        }
                    }
                }
                views.collect_view(cx)
            }
        }}
    }
}
