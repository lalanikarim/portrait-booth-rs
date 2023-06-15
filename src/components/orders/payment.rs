use leptos::*;

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use stripe::PaymentLink;
use stripe::CreatePaymentLinkLineItems; use stripe::CreatePaymentLink;       use stripe::Client;
    }
}

#[server(CreatePaymentLinkRequest, "/api")]
pub async fn create_payment_link_request(cx: Scope) -> Result<String, ServerFnError> {
    let secret = dotenv!("STRIPE_KEY");
    let client = Client::new(secret);
    let price = dotenv!("PHOTO_PRICING_ID").to_string();
    let payment_link = PaymentLink::create(
        &client,
        CreatePaymentLink::new(vec![CreatePaymentLinkLineItems {
            quantity: 5,
            price,
            ..Default::default()
        }]),
    )
    .await
    .unwrap();
    log!("Payment URL: {}", payment_link.url);
    Ok(payment_link.url)
}

#[component]
pub fn Payment(cx: Scope) -> impl IntoView {
    let payment_link_request_action = create_server_action::<CreatePaymentLinkRequest>(cx);
    payment_link_request_action.dispatch(CreatePaymentLinkRequest {});
    view! {
        cx,
        <h2>"Payment"</h2>
    }
}
