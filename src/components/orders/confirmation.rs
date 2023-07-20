use leptos::{ev::MouseEvent, *};
use leptos_router::*;
use serde::{Deserialize, Serialize};

use crate::{
    components::util::loading::Loading,
    models::{order::OrderStatus, user_order::UserOrder},
};

#[server(StoreStripeConfirmation, "/api")]
pub async fn store_stripe_confirmation(
    cx: Scope,
    params: ConfirmationParams,
) -> Result<UserOrder, ServerFnError> {
    let ConfirmationParams {
        order_ref,
        payment_ref,
    } = params;
    match crate::pool(cx) {
        Err(e) => Err(e),
        Ok(pool) => {
            match crate::models::order::Order::get_by_order_confirmation(order_ref.clone(), &pool)
                .await
            {
                Err(e) => Err(e),
                Ok(None) => Err(ServerFnError::Args("Invalid Order Reference".to_string())),
                Ok(Some(order)) => {
                    match crate::models::order::Order::update_order_confirmation(
                        order_ref.clone(),
                        payment_ref,
                        &pool,
                    )
                    .await
                    {
                        Err(e) => Err(e),
                        Ok(false) => Err(ServerFnError::ServerError(
                            "Unable to save order confirmation".to_string(),
                        )),
                        Ok(true) => UserOrder::get_by_order_id(order.id, &pool).await,
                    }
                }
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
        move |_| store_stripe_confirmation(cx, params.get()),
    );
    let on_click = move |_: MouseEvent| {
        let navigate = use_navigate(cx);
        _ = navigate("/", Default::default());
    };
    view! { cx,
        <Suspense fallback=move || {
            view! { cx, <Loading/> }
        }>
            <div class="container">
                <h2 class="header">"Payment Confirmation"</h2>
                {move || {
                    match order_resource.read(cx) {
                        None => view! { cx, <Loading/> }.into_view(cx),
                        Some(response) => {
                            match response {
                                Err(e) => {
                                    view! { cx,
                                        <div class="error">
                                            "Error encountered: " {e.to_string()}
                                        </div>
                                    }
                                        .into_view(cx)
                                }
                                Ok(order) => {
                                    view! { cx,
                                        <div>"Payment confirmed"</div>
                                        <div class="flex flex-row text-left">
                                            <div class="w-1/2">"Order #"</div>
                                            <div class="font-bold">{order.id}</div>
                                        </div>
                                        <div class="flex flex-row text-left">
                                            <div class="w-1/2">"No of Photos"</div>
                                            <div class="font-bold">{order.no_of_photos}</div>
                                        </div>
                                        <div class="flex flex-row text-left">
                                            <div class="w-1/2">"Name"</div>
                                            <div class="font-bold">{order.name.clone()}</div>
                                        </div>
                                        <div class="flex flex-row text-left">
                                            <div class="w-1/2">"Email"</div>
                                            <div class="font-bold">{order.email.clone()}</div>
                                        </div>
                                        <div class="flex flex-row text-left">
                                            <div class="w-1/2">"Phone"</div>
                                            <div class="font-bold">
                                                {order.phone.clone().unwrap_or("".to_string())}
                                            </div>
                                        </div>
                                        <div class="flex flex-row text-left">
                                            <div class="w-1/2">"Status"</div>
                                            <div class="font-bold">
                                                {if order.status == OrderStatus::PaymentPending {
                                                    format!("{:?} ({:?})", order.status, order.mode_of_payment)
                                                } else {
                                                    format!("{:?}", order.status)
                                                }}
                                            </div>
                                        </div>
                                        <div class="flex flex-row text-left">
                                            <div class="w-1/2">"Order total"</div>
                                            <div class="font-bold">"$" {order.order_total}</div>
                                        </div>
                                    }
                                        .into_view(cx)
                                }
                            }
                        }
                    }
                }}
                <button on:click=on_click>"Back"</button>
            </div>
        </Suspense>
    }
}
