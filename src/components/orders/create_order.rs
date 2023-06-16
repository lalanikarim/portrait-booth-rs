use leptos::{
    ev::{MouseEvent, SubmitEvent},
    *,
};

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
#[server(GetUnitPrice, "/api")]
pub async fn get_unit_price(cx: Scope) -> Result<u64, ServerFnError> {
    dotenv!("PHOTO_UNIT_PRICE")
        .parse::<u64>()
        .map_err(|e| crate::to_server_fn_error(e))
}
#[component]
pub fn CreateOrder(cx: Scope, order_created: Action<(), ()>) -> impl IntoView {
    let (error, set_error) = create_signal(cx, "".to_string());
    let (no_of_pics, set_no_of_pics) = create_signal(cx, 0);
    let (unit_price, set_unit_price) = create_signal(cx, 0);
    let total_price = move || format!("${}", no_of_pics.get() * unit_price.get());
    create_resource(
        cx,
        move || no_of_pics.get(),
        move |_| async move {
            match get_unit_price(cx).await {
                Ok(p) => set_unit_price.update(|price| *price = p),
                Err(_) => set_error.update(|e| *e = "Error fetching unit price".to_string()),
            }
        },
    );
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
            <form on:submit=on_submit>
                <div class="flex flex-col text-left">
                    <div class="flex flex-col">
                        <label for="no-of-pics">"Number of Pictures"</label>
                        <div class="flex flex-row justify-between">
                            <button class="w-12" on:click=no_pics_decr disabled=disable_decr>
                                "-"
                            </button>
                            <div class="inline-block align-middle">{no_of_pics}</div>
                            <button class="w-12" on:click=no_pics_incr disabled=disable_incr>
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
        </div>
    }
}
