use crate::{models::order::Order, to_server_fn_error};
use leptos::*;
use stripe::*;

#[server(GetPaymentLink, "/api")]
pub async fn get_payment_link(cx: Scope, order: Order) -> Result<String, ServerFnError> {
    let secret_key = dotenvy::var("STRIPE_KEY").expect("STRIPE_KEY env variable should be present");
    if order.no_of_photos < 1 || order.no_of_photos > 3 {
        return Err(ServerFnError::Args(
            "Only 1, 2, or 3 photos can be purchased.".to_string(),
        ));
    }
    let pricing_id_name = format!("PHOTO_PRICING_ID_{}", order.no_of_photos);
    let pricing_id =
        dotenvy::var(pricing_id_name).expect("PHOTO_PRICING_ID env variable should be present");

    let app_url = dotenvy::var("APP_URL").expect("APP_URL env variable should be present");
    let order_ref = order.order_ref.expect("Order Ref should be present");
    let client = Client::new(secret_key);
    let mut create_payment_link_args = CreatePaymentLink::new(vec![CreatePaymentLinkLineItems {
        price: pricing_id,
        quantity: 1,
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
        .map_err(to_server_fn_error)
}
