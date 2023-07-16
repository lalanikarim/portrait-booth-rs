use leptos::{server, ServerFnError};
use lettre::message::header::ContentType;
use lettre::message::MessageBuilder;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};

use crate::to_server_fn_error;

fn get_mailer() -> Result<AsyncSmtpTransport<Tokio1Executor>, ServerFnError> {
    let username = dotenvy::var("SMTP_USERNAME").expect("SMTP_USERNAME should be present");
    let password = dotenvy::var("SMTP_PASSWORD").expect("SMTP_PASSWORD should be present");
    let relay = dotenvy::var("SMTP_RELAY").expect("SMTP_RELAY should be present");

    let creds = Credentials::new(username, password);

    AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(relay.as_str())
        .map_err(to_server_fn_error)
        .map(|r| r.credentials(creds).build())
}

fn email_builder() -> MessageBuilder {
    let from = dotenvy::var("SMTP_FROM_EMAIL").expect("SMTP_FROM_EMAIL should be present");
    let reply_to =
        dotenvy::var("SMTP_REPLY_TO_EMAIL").expect("SMTP_REPLY_TO_EMAIL should be present");
    Message::builder()
        .from(from.parse().unwrap())
        .reply_to(reply_to.parse().unwrap())
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
        .map_err(to_server_fn_error)
}

#[server(SendProcessedEmail, "/api")]
pub async fn send_processed(
    to: String,
    name: String,
    links: Vec<String>,
) -> Result<bool, ServerFnError> {
    let reply_to =
        dotenvy::var("SMTP_REPLY_TO_EMAIL").expect("SMTP_REPLY_TO_EMAIL should be present");
    let from_name = dotenvy::var("EMAIL_FROM_NAME").expect("EMAIL_FROM_NAME should be present");
    let mailer = match get_mailer() {
        Ok(mailer) => mailer,
        Err(e) => return Err(e),
    };

    let links = links
        .into_iter()
        .enumerate()
        .map(|(i, link)| format!(r#"<p><a href="{link}">Portrait {}</a></p>"#, i + 1))
        .collect::<String>();

    let email = match email_builder()
        .to(to.parse().unwrap())
        .subject("Your portraits are ready")
        .header(ContentType::TEXT_HTML)
        .body(format!(r#"
        <p>Dear {name},</p>
        
        <p>Your portraits are ready.  To download your portraits, please click on the link(s) below.</p>
        <p><strong>The below links will expire in 5 days.</strong></p>

        {links}

        <p>Please download these portraits before the links expire. 
        If you have any questions about your portraits, please send an email to {reply_to}.
        We will try and get back with you within a few days.</p>

        <p>Please DO NOT respond to this email as this mailbox is not monitored.</p>

        <p>Regards,</p>

        <p>{from_name}</p>
        "#)) {
            Err(e) => return Err(to_server_fn_error(e)),
            Ok(email) => email
        };

    mailer
        .send(email)
        .await
        .map(|_| true)
        .map_err(to_server_fn_error)
}
