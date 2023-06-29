use leptos::*;
use web_sys::MouseEvent;

use crate::{
    components::util::loading::Loading,
    models::{order::OrderStatus, user_order::UserOrder},
};

#[component]
pub fn SearchResults(
    cx: Scope,
    orders: RwSignal<Option<Result<Vec<UserOrder>, ServerFnError>>>,
) -> impl IntoView {
    view! {cx,

        <div class="container-lg">
            <h2 class="header">"Search Results"</h2>
            <Suspense fallback=move || {
                view! { cx, <Loading /> }
            }>
                {move || {
                    let result = orders.get();
                    match result {
                        None => {
                            view! { cx, <div>"Use form above to search orders"</div> }
                                .into_view(cx)
                        }
                        Some(result) => {
                            match result {
                                Err(e) => {
                                    view! { cx, <div class="red">{e.to_string()}</div> }
                                        .into_view(cx)
                                }
                                Ok(results) => {
                                    view! { cx,
                                        <table class="table-auto w-full broder-collapse border border-slate-400">
                                            <thead class="bg-slate-50">
                                                <tr>
                                                    <th class="border border-slate-300 p-1 w-1/12">"#"</th>
                                                    <th class="border border-slate-300 p-1 w-3/6">"Name"</th>
                                                    <th class="border border-slate-300 p-1 w-1/12">"Pics"</th>
                                                    <th class="border border-slate-300 p-1 w-2/6">"Status"</th>
                                                </tr>
                                            </thead>
                                            <tbody>
                                                {move || {
                                                    results
                                                        .iter()
                                                        .map(move |o| {
                                                            let user_order = o.clone();
                                                            view! { cx, <UserOrderRow user_order/> }
                                                        })
                                                        .collect_view(cx)
                                                }}
                                            </tbody>
                                        </table>
                                    }
                                        .into_view(cx)
                                }
                            }
                        }
                    }
                }}
            </Suspense>
        </div>
    }
}
#[component]
pub fn UserOrderRow(cx: Scope, user_order: UserOrder) -> impl IntoView {
    let set_order = use_context::<WriteSignal<Option<UserOrder>>>(cx)
        .expect("Set Order Search should be present");
    let o = user_order.clone();
    let status = move || match o.status {
        OrderStatus::PaymentPending => format!("{:?} ({:?})", o.status, o.mode_of_payment),
        _ => format!("{:?}", o.status),
    };
    let on_click = move |_: MouseEvent| set_order.set(Some(user_order.clone()));
    view! { cx,
        <tr on:click=on_click>
            <td class="border border-slate-300 p-1">{o.id}</td>
            <td class="border border-slate-300 p-1 text-left">
                <div>{o.name}</div>
                <div>{o.email}</div>
                <div>{o.phone.unwrap_or("".to_string())}</div>
            </td>
            <td class="border border-slate-300 p-1">{o.no_of_photos}</td>
            <td class="border border-slate-300 p-1">{status}</td>
        </tr>
    }
}
