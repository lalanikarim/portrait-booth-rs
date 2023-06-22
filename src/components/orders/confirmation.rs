use leptos::{ev::MouseEvent, *};
use leptos_router::*;
use serde::{Deserialize, Serialize};

use crate::models::order::Order;

#[server(StoreStripeConfirmation, "/api")]
pub async fn store_stripe_confirmation(
    cx: Scope,
    params: ConfirmationParams,
) -> Result<Order, ServerFnError> {
    let pool = crate::pool(cx).expect("Pool should be present");
    let ConfirmationParams {
        order_ref,
        payment_ref,
    } = params;
    match Order::get_by_order_confirmation(order_ref.clone(), &pool).await {
        Err(e) => Err(e),
        Ok(None) => Err(ServerFnError::Args("Invalid Order Reference".to_string())),
        Ok(Some(order)) => {
            match Order::update_order_confirmation(order_ref.clone(), payment_ref, &pool).await {
                Err(e) => Err(e),
                Ok(false) => Err(ServerFnError::ServerError(
                    "Unable to save order confirmation".to_string(),
                )),
                Ok(true) => Ok(order),
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfirmationParams {
    pub order_ref: String,
    pub payment_ref: String,
}

#[component]
pub fn Confirmation(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);
    let params = move || {
        params.with(|params| ConfirmationParams {
            order_ref: params
                .get("order_ref")
                .map(|param| param.to_owned())
                .unwrap_or("invalid".to_string()),
            payment_ref: params
                .get("payment_ref")
                .map(|param| param.to_owned())
                .unwrap_or("invalid".to_string()),
        })
    };
    let params = Signal::derive(cx, params);
    let order_resource = create_resource(
        cx,
        || (),
        move |_| async move {
            log!("Sending: {:#?}", params.get());
            store_stripe_confirmation(cx, params.get()).await
        },
    );
    let on_click = move |_: MouseEvent| {
        let navigate = use_navigate(cx);
        _ = navigate("/", Default::default());
    };
    view! { cx,
        <div class="container">
            <h2 class="header">"Payment Confirmation"</h2>
            <Suspense fallback=move || {
                view! { cx, <div>"Loading..."</div> }
            }>
                {move || {
                    match order_resource.read(cx) {
                        None => {
                            view! { cx, <div>"Loading..."</div> }
                                .into_view(cx)
                        }
                        Some(response) => {
                            match response {
                                Err(e) => {
                                    view! { cx, <div class="error">"Error encountered: " {e.to_string()}</div> }
                                        .into_view(cx)
                                }
                                Ok(_order) => {
                                    //set_order.set(Some(order));
                                    view! { cx, <div>"Payment confirmed"</div> }
                                        .into_view(cx)
                                }
                            }
                        }
                    }
                }} <button on:click=on_click>"Back"</button>
            </Suspense>
        </div>
    }
}
