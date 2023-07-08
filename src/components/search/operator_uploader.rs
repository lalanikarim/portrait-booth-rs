use leptos::*;

use crate::{
    components::{
        files::{
            file_list::FileList,
            uploader::{Uploader, UploaderMode},
        },
        util::{empty_view::EmptyView, loading::Loading},
    },
    models::order::{Order, OrderStatus},
    models::user_order::UserOrder,
};

#[component]
pub fn OperatorUploader(
    cx: Scope,
    order_resource: Resource<Option<UserOrder>, Result<Option<Order>, ServerFnError>>,
) -> impl IntoView {
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
                                    <FileList order=order.clone() mode />
                                    {move || {
                                        if order.clone().status == OrderStatus::Uploading {
                                            view! { cx,
                                                <Uploader order=order.clone() mode order_resource/>
                                            }
                                        } else {
                                            view! { cx, <EmptyView/> }
                                        }
                                    }}
                                }
                                    .into_view(cx)
                            }
                            Ok(None) => {
                                view! { cx, <EmptyView/> }
                            }
                            Err(e) => {
                                view! { cx, <div class="red">{e.to_string()}</div> }.into_view(cx)
                            }
                        }
                    }
                }
            }}
        </Suspense>
    }
}
