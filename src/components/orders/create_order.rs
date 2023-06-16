use leptos::{
    ev::{MouseEvent, SubmitEvent},
    *,
};

use crate::components::error_template::ErrorTemplate;

use super::UnitPrice;

#[server(CreateOrderRequest, "/api")]
pub async fn create_order_request(cx: Scope, no_of_photos: u64) -> Result<bool, ServerFnError> {
    use crate::models::{order::Order, user::User};
    let pool = crate::pool(cx).expect("Pool should be present");
    let auth = crate::auth::auth(cx).expect("Auth should be present");
    let User {
        id: customer_id, ..
    } = auth.current_user.expect("No logged in user");
    Order::create(customer_id, no_of_photos, &pool)
        .await
        .map(|_| true)
}
#[component]
pub fn CreateOrder(cx: Scope, order_created: Action<(), ()>) -> impl IntoView {
    let (error, set_error) = create_signal(cx, "".to_string());
    let (no_of_pics, set_no_of_pics) = create_signal(cx, 0);
    let unit_price_resource = use_context::<Resource<(), Result<UnitPrice, ServerFnError>>>(cx)
        .expect("Unit Price Resource should be present");
    let create_order_action = create_server_action::<CreateOrderRequest>(cx);
    create_effect(cx, move |_| {
        let Some(result) = create_order_action.value().get() else { return;};
        match result {
            Ok(_) => {
                set_no_of_pics.update(|v| *v = 0);
                set_error.update(|e| *e = "".to_string());
            }
            Err(e) => set_error.update(|er| *er = e.to_string()),
        };
        order_created.dispatch(());
    });
    let disable_create = move || create_order_action.pending().get() || no_of_pics.get() == 0;
    let disable_decr = move || no_of_pics.get() == 0;
    let disable_incr = move || no_of_pics.get() >= 20;
    let create_title = move || {
        if create_order_action.pending().get() {
            "Creating..."
        } else {
            "Create"
        }
    };
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let no_of_photos = no_of_pics.get();
        if no_of_photos > 0 {
            create_order_action.dispatch(CreateOrderRequest { no_of_photos });
        }
    };
    let no_pics_incr = move |ev: MouseEvent| {
        ev.prevent_default();
        if no_of_pics.get() <= 20 {
            set_no_of_pics.update(|n| *n += 1);
        }
    };
    let no_pics_decr = move |ev: MouseEvent| {
        ev.prevent_default();
        if no_of_pics.get() > 0 {
            set_no_of_pics.update(|n| *n -= 1);
        }
    };
    view! { cx,
        <div class="container">
            <h2 class="header">"Create Order"</h2>
            {move || {
                match unit_price_resource.read(cx) {
                    None => {
                        view! { cx, <div>"Loading..."</div> }
                            .into_view(cx)
                    }
                    Some(p) => {
                        match p {
                            Err(e) => {
                                view! { cx, <div class="error">"Server Error: " {e.to_string()}</div> }
                                    .into_view(cx)
                            }
                            Ok(UnitPrice(unit_price)) => {
                                let total_price = format!("${}", unit_price * no_of_pics.get());
                                view! { cx,
                                    <form on:submit=on_submit>
                                        <div class="flex flex-col text-left">
                                            <div class="flex flex-col">
                                                <label for="no-of-pics">"Number of Pictures"</label>
                                                <div class="flex flex-row justify-between">
                                                    <button
                                                        class="w-12"
                                                        on:click=no_pics_decr
                                                        disabled=disable_decr
                                                    >
                                                        "-"
                                                    </button>
                                                    <div class="inline-block align-middle">{no_of_pics}</div>
                                                    <button
                                                        class="w-12"
                                                        on:click=no_pics_incr
                                                        disabled=disable_incr
                                                    >
                                                        "+"
                                                    </button>
                                                </div>
                                            </div>
                                            <div class="flex flex-col mt-2">
                                                <label>"Total"</label>
                                                <div class="text-right">{total_price}</div>
                                                <span class="error">{error}</span>
                                            </div>
                                            <div class="text-center mt-8">
                                                <button class="w-40" type="submit" disabled=disable_create>
                                                    {create_title}
                                                </button>
                                            </div>
                                        </div>
                                    </form>
                                }
                                    .into_view(cx)
                            }
                        }
                    }
                }
            }}
        </div>
    }
}
