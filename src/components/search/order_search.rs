use crate::components::search::search_results::SearchResults;
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
        <SearchResults orders=order_search_action.value() />
    }
}
