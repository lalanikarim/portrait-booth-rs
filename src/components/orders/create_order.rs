use leptos::{ev::SubmitEvent, *};

use crate::{
    components::util::{loading::Loading, show_error::ShowError},
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
    let (pool,User{id: customer_id,..}) = crate::server::pool_and_current_user(cx)?; 
    let Some(order) = Order::create(customer_id, no_of_photos, &pool).await? else {
        return Ok(None);
    };
    UserOrder::get_by_order_id(order.id, &pool).await.map(Some)
}

#[component]
pub fn CreateOrder(cx: Scope) -> impl IntoView {
    let Some(set_order) = use_context::<WriteSignal<Option<UserOrder>>>(cx) else {
        return view! { cx, <div class="red">"Set_order write signal should be present"</div> };
    };
    let (show_error, set_error) = create_signal(cx, None);
    let (no_of_pics, set_no_of_pics) = create_signal(cx, None);
    let Some(unit_price_resource) = use_context::<Resource<(), Result<Pricing, ServerFnError>>>(cx) else { 
        return view! { cx, <div class="red">"Unit Price Resource should be present"</div> };
    };
    let create_order_action = create_server_action::<CreateOrderRequest>(cx);
    let disable_create = move || create_order_action.pending().get() || no_of_pics.get().is_none();
    let create_title = move || {
        if create_order_action.pending().get() {
            "Creating..."
        } else {
            "Continue"
        }
    };
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let Some(no_of_photos) = no_of_pics.get() else {return;};
        if no_of_photos > 0 {
            create_order_action.dispatch(CreateOrderRequest { no_of_photos });
            set_no_of_pics.set(None);
        }
    };
    let get_price = move |zero_price: u64, unit_price: u64, qty: u64| unit_price * qty + zero_price;
    let get_btn_class = move |pics| {
        no_of_pics
            .get()
            .map(|p| if p == pics { "green" } else { "royal-blue" })
            .unwrap_or("royal-blue")
    };
    let total_price = move |base_price, unit_price| {
        format!(
            "${}",
            no_of_pics
                .get()
                .map(|no_of_pics| unit_price * no_of_pics + base_price)
                .unwrap_or(0)
        )
    };
    create_effect(cx, move |_| {
        let Some(result) = create_order_action.value().get() else { return;};
        match result {
            Ok(order) => {
                set_error.set(None);
                set_no_of_pics.set(None);
                set_order.set(order);
            }
            Err(e) => set_error.set(Some(e.to_string())),
        };
    });
    view! { cx,
        <div class="container">
            <h2 class="header">"Create Order"</h2>
            {move || {
                match unit_price_resource.read(cx) {
                    None => {
                        view! { cx, <Loading/> }
                    }
                    Some(Err(e)) => {
                        view! { cx, <ShowError error=e.to_string()/> }
                    }
                    Some(Ok(Pricing { unit_price, base_price })) => {
                        view! { cx,
                            <form on:submit=on_submit>
                                <div class="flex flex-col text-left">
                                    <div class="flex flex-col gap-5 w-40 mx-auto">
                                        <div class="text-center">"Number of Pictures"</div>
                                        <button
                                            class="w-1/2"
                                            class=move || get_btn_class(1)
                                            on:click=move |_| set_no_of_pics.set(Some(1))
                                        >
                                            {move || {
                                                let qty = 1;
                                                format!(
                                                    "{} for ${}", qty, get_price(base_price, unit_price, qty)
                                                )
                                            }}
                                        </button>
                                        <button
                                            class="w-1/2"
                                            class=move || get_btn_class(2)
                                            on:click=move |_| set_no_of_pics.set(Some(2))
                                        >
                                            {move || {
                                                let qty = 2;
                                                format!(
                                                    "{} for ${}", qty, get_price(base_price, unit_price, qty)
                                                )
                                            }}
                                        </button>
                                        <button
                                            class="w-1/2"
                                            class=move || get_btn_class(3)
                                            on:click=move |_| set_no_of_pics.set(Some(3))
                                        >
                                            {move || {
                                                let qty = 3;
                                                format!(
                                                    "{} for ${}", qty, get_price(base_price, unit_price, qty)
                                                )
                                            }}
                                        </button>
                                        <div class="text-center">
                                            "Total: " {total_price(base_price, unit_price)}
                                        </div>
                                    </div>
                                    <div class="text-center mt-8">
                                        <button class="w-40" type="submit" disabled=disable_create>
                                            {create_title}
                                        </button>
                                        {show_error}
                                    </div>
                                </div>
                            </form>
                        }
                            .into_view(cx)
                    }
                }
            }}
        </div>
    }
}
