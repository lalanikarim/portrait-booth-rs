use leptos::*;

use crate::{models::{user_order::UserOrder, order::Order}, components::{util::{loading::Loading, empty_view::EmptyView}, files::{uploader::{UploaderMode, Uploader}, file_list::FileList}}};

#[component]
pub fn OperatorUploader(cx:Scope, order_resource:Resource<Option<UserOrder>,Result<Option<Order>,ServerFnError>>) -> impl IntoView { 
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
                                view! { cx, <div class="red">{e.to_string()}</div> }.into_view(cx)
                            }
                        }
                    }
                }
            }}
        </Suspense>
    }
}
