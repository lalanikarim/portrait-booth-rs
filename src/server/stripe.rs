use crate::{models::order::Order, to_server_fn_error};
use leptos::*;
use stripe::*;

#[server(GetPaymentLink, "/api")]
pub async fn get_payment_link(cx: Scope, order: Order) -> Result<String, ServerFnError> {
    let secret_key = dotenv!("STRIPE_KEY");
    let pricing_id = dotenv!("PHOTO_PRICING_ID");

    let app_url = dotenv!("APP_URL");
    let order_ref = order.order_ref.expect("Order Ref should be present");
    let client = Client::new(secret_key);
    let mut create_payment_link_args = CreatePaymentLink::new(vec![CreatePaymentLinkLineItems {
        price: pricing_id.into(),
        quantity: order.no_of_photos,
        ..Default::default()
    }]);
    create_payment_link_args.after_completion = Some(CreatePaymentLinkAfterCompletion {
        redirect: Some(CreatePaymentLinkAfterCompletionRedirect {
            url: format!(
                "{}/confirmation/{}/{}",
                app_url, order_ref, "{CHECKOUT_SESSION_ID}"
            ),
        }),
        type_: CreatePaymentLinkAfterCompletionType::Redirect,
        ..Default::default()
    });
    PaymentLink::create(&client, create_payment_link_args)
        .await
        .map(|link| link.url)
        .map_err(|e: StripeError| to_server_fn_error(e))
}
