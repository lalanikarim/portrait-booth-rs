use leptos::{server, ServerFnError};
use lettre::message::header::ContentType;
use lettre::message::MessageBuilder;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};

use crate::to_server_fn_error;

fn get_mailer() -> Result<AsyncSmtpTransport<Tokio1Executor>, ServerFnError> {
    let username = dotenv::var("SMTP_USERNAME").expect("SMTP_USERNAME should be present");
    let password = dotenv::var("SMTP_PASSWORD").expect("SMTP_PASSWORD should be present");
    let relay = dotenv::var("SMTP_RELAY").expect("SMTP_RELAY should be present");

    let creds = Credentials::new(username, password);

    AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(relay.as_str())
        .map_err(|e| to_server_fn_error(e))
        .map(|r| r.credentials(creds).build())
}

fn email_builder() -> MessageBuilder {
    let from = dotenv::var("SMTP_FROM_EMAIL").expect("SMTP_FROM_EMAIL should be present");
    Message::builder()
        .from(from.parse().unwrap())
        .reply_to(from.parse().unwrap())
}

#[server(SendOtpEmail, "/api")]
pub async fn send_otp(to: String, otp: String) -> Result<bool, ServerFnError> {
    let mailer = match get_mailer() {
        Ok(mailer) => mailer,
        Err(e) => return Err(e),
    };

    let email = match email_builder()
        .to(to.parse().unwrap())
        .subject("Login Code for Portrait Booth")
        .header(ContentType::TEXT_PLAIN)
        .body(format!("Your login code is: {}.\nThis code will expire soon after which you will need to request a new login code.",otp)) {
            Err(e) => return Err(to_server_fn_error(e)),
            Ok(email) => email
        };

    mailer
        .send(email)
        .await
        .map(|_| true)
        .map_err(|e| to_server_fn_error(e))
}
