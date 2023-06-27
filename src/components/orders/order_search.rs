use crate::models::order::OrderStatus;
use crate::models::user_order::OrderSearchForm;
use crate::models::user_order::UserOrder;
use leptos::html::Input;
use leptos::*;
use web_sys::SubmitEvent;

#[server(OrderSearchRequest, "/api")]
pub async fn order_search_request(
    cx: Scope,
    form: OrderSearchForm,
) -> Result<Vec<UserOrder>, ServerFnError> {
    let pool = crate::pool(cx).expect("Pool should exist");
    UserOrder::search_orders(form, &pool).await
}

#[component]
pub fn OrderSearch(cx: Scope) -> impl IntoView {
    let order_no_input = create_node_ref::<Input>(cx);
    let name_input = create_node_ref::<Input>(cx);
    let email_input = create_node_ref::<Input>(cx);
    let phone_input = create_node_ref::<Input>(cx);
    let order_search_action = create_server_action::<OrderSearchRequest>(cx);
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let order_no = order_no_input
            .get()
            .expect("Order no should be present")
            .value();
        let name = name_input.get().expect("Name should be present").value();
        let email = email_input.get().expect("Email should be present").value();
        let phone = phone_input.get().expect("Phone should be present").value();
        let order_no = if order_no.len() > 0 {
            order_no.parse().ok()
        } else {
            None
        };
        let name = if name.len() > 0 { Some(name) } else { None };
        let email = if email.len() > 0 { Some(email) } else { None };
        let phone = if phone.len() > 0 { Some(phone) } else { None };
        let form = OrderSearchForm {
            order_no,
            name,
            email,
            phone,
        };
        order_search_action.dispatch(OrderSearchRequest { form });
    };
    view! { cx,
        <div class="container">
            <h2 class="header">"Search Orders"</h2>
            <form on:submit=on_submit>
                <div class="flex flex-col">
                    <div class="flex justify-between">
                        <label class="p-2" for="customer_order_no">
                            "Order #"
                        </label>
                        <input id="customer_order_no" _ref=order_no_input type="number" min="1"/>
                    </div>
                    <div class="flex justify-between">
                        <label class="p-2" for="customer_name">
                            "Name"
                        </label>
                        <input id="customer_name" _ref=name_input type="text"/>
                    </div>
                    <div class="flex justify-between">
                        <label class="p-2" for="customer_email">
                            "Email"
                        </label>
                        <input id="customer_email" _ref=email_input type="email"/>
                    </div>
                    <div class="flex justify-between">
                        <label class="p-2" for="customer_phone">
                            "Phone"
                        </label>
                        <input id="customer_phone" _ref=phone_input type="text"/>
                    </div>
                    <div class="flex justify-around mt-4">
                        <button class="red" type="reset">
                            "Reset"
                        </button>
                        <button type="submit">"Search"</button>
                    </div>
                </div>
            </form>
        </div>
        <div class="container-lg">
            <h2 class="header">"Search Results"</h2>
            <Suspense fallback=move || {
                view! { cx, <div>"Loading..."</div> }
            }>
                {move || {
                    let result = order_search_action.value().get();
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
    let o = user_order;
    let status = move || match o.status {
        OrderStatus::PaymentPending => format!("{:?} ({:?})", o.status, o.mode_of_payment),
        _ => format!("{:?}", o.status),
    };
    view! { cx,
        <tr>
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
