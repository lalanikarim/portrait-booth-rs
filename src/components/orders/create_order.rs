use leptos::{ev::SubmitEvent, *};

use crate::{
    components::util::loading::Loading,
    models::{pricing::Pricing, user_order::UserOrder},
};

#[server(CreateOrderRequest, "/api")]
pub async fn create_order_request(
    cx: Scope,
    no_of_photos: u64,
) -> Result<Option<UserOrder>, ServerFnError> {
    if !(1..=3).contains(&no_of_photos) {
        return Err(ServerFnError::Args(
            "Only 1, 2, or 3 photos can be ordered".into(),
        ));
    }
    use crate::models::{order::Order, user::User};
    match crate::server::pool_and_current_user(cx) {
        Err(e) => Err(e),
        Ok((
            pool,
            User {
                id: customer_id, ..
            },
        )) => match Order::create(customer_id, no_of_photos, &pool).await {
            Err(e) => Err(e),
            Ok(None) => Ok(None),
            Ok(Some(order)) => UserOrder::get_by_order_id(order.id, &pool)
                .await
                .map(Some),
        },
    }
}

#[component]
pub fn CreateOrder(cx: Scope, order_created: Action<(), ()>) -> impl IntoView {
    let set_order = use_context::<WriteSignal<Option<UserOrder>>>(cx)
        .expect("Set_order write signal should be present");
    let (error, set_error) = create_signal(cx, "".to_string());
    let (no_of_pics, set_no_of_pics) = create_signal(cx, None);
    let unit_price_resource = use_context::<Resource<(), Result<Pricing, ServerFnError>>>(cx)
        .expect("Unit Price Resource should be present");
    let create_order_action = create_server_action::<CreateOrderRequest>(cx);
    create_effect(cx, move |_| {
        let Some(result) = create_order_action.value().get() else { return;};
        match result {
            Ok(order) => {
                set_error.update(|e| *e = "".to_string());
                set_no_of_pics.set(None);
                set_order.set(order);
            }
            Err(e) => set_error.update(|er| *er = e.to_string()),
        };
        order_created.dispatch(());
    });
    let disable_create = move || create_order_action.pending().get() || no_of_pics.get().is_none();
    let create_title = move || {
        if create_order_action.pending().get() {
            "Creating..."
        } else {
            "Create"
        }
    };
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let Some(no_of_photos) = no_of_pics.get() else {return;};
        if no_of_photos > 0 {
            create_order_action.dispatch(CreateOrderRequest { no_of_photos });
        }
    };
    let get_price = move |zero_price: u64, unit_price: u64, qty: u64| unit_price * qty + zero_price;
    let get_btn_class = move |pics| {
        no_of_pics
            .get()
            .map(|p| if p == pics { "green" } else { "royal-blue" })
            .unwrap_or("royal-blue")
    };
    view! { cx,
        <div class="container">
            <h2 class="header">"Create Order"</h2>
            {move || {
                match unit_price_resource.read(cx) {
                    None => {
                        view! { cx, <Loading /> }
                            .into_view(cx)
                    }
                    Some(p) => {
                        match p {
                            Err(e) => {
                                view! { cx, <div class="error">"Server Error: " {e.to_string()}</div> }
                                    .into_view(cx)
                            }
                            Ok(Pricing{base_price, unit_price}) => {
                                let total_price = format!(
                                    "${}", no_of_pics.get().map(| no_of_pics | unit_price *
                                    no_of_pics + base_price).unwrap_or(0)
                                );
                                view! { cx,
                                    <form on:submit=on_submit>
                                        <div class="flex flex-col text-left">
                                            <div class="flex flex-col">
                                                <label for="no-of-pics">"Number of Pictures"</label>
                                                <div class="flex flex-row justify-between">
                                                    <button
                                                        class="w-1/4"
                                                        class=move || get_btn_class(1)
                                                        on:click=move |_| set_no_of_pics.set(Some(1))
                                                    >
                                                        {move || {
                                                            let qty = 1;
                                                            format!("{} for ${}", qty, get_price(base_price, unit_price, qty))
                                                        }}
                                                    </button>
                                                    <button
                                                        class="w-1/4"
                                                        class=move || get_btn_class(2)
                                                        on:click=move |_| set_no_of_pics.set(Some(2))
                                                    >
                                                        {move || {
                                                            let qty = 2;
                                                            format!("{} for ${}", qty, get_price(base_price, unit_price, qty))
                                                        }}
                                                    </button>
                                                    <button
                                                        class="w-1/4"
                                                        class=move || get_btn_class(3)
                                                        on:click=move |_| set_no_of_pics.set(Some(3))
                                                    >
                                                        {move || {
                                                            let qty = 3;
                                                            format!("{} for ${}", qty, get_price(base_price, unit_price, qty))
                                                        }}
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
