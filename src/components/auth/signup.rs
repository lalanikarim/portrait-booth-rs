use leptos::{ev::SubmitEvent, html::Input, *};
use leptos_router::use_navigate;
use serde::{Deserialize, Serialize};

use crate::components::auth::login_otp::LoginOtpRequest;

#[derive(Serialize, Deserialize)]
pub enum SignupResponse {
    Success,
    EmailAlreadyUsed,
    PhoneAlreadyUsed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignupForm {
    pub fullname: String,
    pub email: String,
    pub phone: Option<String>,
    pub password: Option<String>,
}
fn validate_password(password: String, confirm_password: String) -> Result<(), Vec<String>> {
    let mut error: Vec<String> = Vec::new();
    if password != confirm_password {
        error.push("Passwords don't match.".into());
    }
    if password.len() < 8 {
        error.push("Must contain at least 8 characters.".into());
    }
    if password.find(|c: char| c.is_lowercase()).is_none() {
        error.push("Must contain at least one lowercase letter.".into());
    }
    if password.find(|c: char| c.is_uppercase()).is_none() {
        error.push("Must contain at least one uppercase letter.".into());
    }
    if password.find(|c: char| c.is_numeric()).is_none() {
        error.push("Must contain at least one number.".into());
    }
    if password.find(|c: char| "!@#$%^&*".contains(c)).is_none() {
        error.push("Must contain at least one symbol: !@#$%^&*".into());
    }
    if error.len() == 0 {
        Ok(())
    } else {
        Err(error)
    }
}

/*
fn validate_email(email: String) -> Result<Option<String>, Vec<String>> {
    todo!()
}

fn validate_phone(phone: String) -> Result<Option<String>, Vec<String>> {
    todo!()
}
*/

#[server(SignupRequest, "/api")]
pub async fn signup_request(cx: Scope, form: SignupForm) -> Result<SignupResponse, ServerFnError> {
    use crate::pool;

    use crate::models::user::Role;
    use totp_rs::*;

    let pool = pool(cx)?;
    match sqlx::query_scalar!("SELECT COUNT(1) FROM `users` where email = ?", form.email)
        .fetch_one(&pool)
        .await
    {
        Ok(1) => {
            return Ok(SignupResponse::EmailAlreadyUsed);
        }
        Err(e) => {
            return Err(ServerFnError::ServerError(e.to_string()));
        }
        _ => (),
    };

    let Secret::Encoded(otp_secret) = Secret::generate_secret().to_encoded() else {
        return Err(ServerFnError::ServerError("Unable to generate OTP Secret".into()));
    };
    let password_hash = bcrypt::hash(form.password.unwrap_or(otp_secret.clone()), 12).unwrap();

    sqlx::query!(
        "INSERT INTO users (name, email, phone, password_hash,otp_secret, role) values (?, ?, ?, ?,?, ?)",
        form.fullname,
        form.email,
        form.phone,
        password_hash,
        otp_secret,
        Role::Anonymous
    )
    .execute(&pool)
    .await
    .map(|_| SignupResponse::Success)
    .map_err(|e| ServerFnError::ServerError(e.to_string()))
}
#[component]
pub fn signup(
    cx: Scope,
    #[prop(default = true)] ask_password: bool,
    #[prop(default = false)] otp_on_success: bool,
    #[prop(optional)] completed: Option<Action<(), ()>>,
) -> impl IntoView {
    let (errors, set_errors) = create_signal::<Vec<String>>(cx, Vec::new());
    let fullname_input = create_node_ref::<Input>(cx);
    let email_input = create_node_ref::<Input>(cx);
    let phone_input = create_node_ref::<Input>(cx);
    let password_input = create_node_ref::<Input>(cx);
    let confirm_password_input = create_node_ref::<Input>(cx);
    let error_items = move || errors.get().join(" ");
    let signup_action_fn = move |form: &SignupForm| {
        let form = form.clone();
        async move {
            let navigate = use_navigate(cx);
            match signup_request(cx, form.clone()).await {
                Err(e) => {
                    let err_str = e.to_string();
                    set_errors.update(|err| err.push(err_str));
                }
                Ok(result) => {
                    match result {
                        SignupResponse::Success => {
                            if otp_on_success {
                                let SignupForm { email, .. } = form;
                                let url = format!(
                                    "/otp?email={}&show_email=false",
                                    email.replace("+", "%2b")
                                );
                                log!("url: {url}");
                                let login_otp_request_action =
                                    create_server_action::<LoginOtpRequest>(cx);
                                login_otp_request_action.dispatch(LoginOtpRequest { email });
                                _ = navigate(url.as_str(), Default::default());
                            } else {
                                _ = navigate("/", Default::default());
                                if let Some(completed) = completed {
                                    completed.dispatch(());
                                }
                            }
                        }
                        SignupResponse::EmailAlreadyUsed => {
                            set_errors.update(|err| err.push("Email already registered".into()));
                        }
                        SignupResponse::PhoneAlreadyUsed => {
                            set_errors.update(|err| err.push("Phone already registered".into()));
                        }
                    };
                }
            }
        }
    };
    let signup_action = create_action(cx, signup_action_fn);
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        set_errors.update(|err| err.clear());
        let fullname = fullname_input
            .get()
            .expect("fullname input should exist")
            .value();
        let email = email_input
            .get()
            .expect("username input should exist")
            .value();
        let phone = phone_input
            .get()
            .expect("username input should exist")
            .value();

        let ok_to_signup = if ask_password {
            let password = password_input
                .get()
                .expect("password input should exist")
                .value();
            let confirm_password = confirm_password_input
                .get()
                .expect("confirm password input should exist")
                .value();
            if let Err(password_error) =
                validate_password(password.clone(), confirm_password.clone())
            {
                set_errors.update(move |err| err.extend_from_slice(&password_error));
                (false, None)
            } else {
                (true, Some(password))
            }
        } else {
            (true, None)
        };
        if let (true, password) = ok_to_signup {
            let phone = if phone.len() > 0 { Some(phone) } else { None };
            let form = SignupForm {
                fullname,
                email,
                phone,
                password,
            };
            signup_action.dispatch(form);
        }
    };
    let disable_controls = move || signup_action.pending().get();

    view! { cx,
        <div class="container">
            <h2 class="header">"Signup"</h2>
            <form on:submit=on_submit>
                <div class="flex flex-col text-left">
                    <div class="flex flex-col">
                        <label for="fullname">"Full Name"</label>
                        <input
                            id="fullname"
                            type="text"
                            disabled=disable_controls
                            node_ref=fullname_input
                            max-length="25"
                            required
                        />
                    </div>
                    <div class="flex flex-col">
                        <label for="email">"Email"</label>
                        <input
                            id="email"
                            type="email"
                            disabled=disable_controls
                            node_ref=email_input
                            required
                            max-length="25"
                        />
                    </div>
                    <div class="flex flex-col">
                        <label for="phone">"Phone"</label>
                        <input
                            id="phone"
                            type="text"
                            disabled=disable_controls
                            node_ref=phone_input
                            max-length="25"
                        />
                    </div>
                    {move || {
                        if ask_password {
                            view! { cx,
                                <div class="flex flex-col mt-2">
                                    <label for="password">"Password"</label>
                                    <input
                                        id="password"
                                        type="password"
                                        disabled=disable_controls
                                        node_ref=password_input
                                        required
                                        max-length="25"
                                    />
                                </div>
                                <div class="flex flex-col mt-2">
                                    <label for="confirm_password">"Confirm Password"</label>
                                    <input
                                        id="confirm_password"
                                        type="password"
                                        disabled=disable_controls
                                        node_ref=confirm_password_input
                                        required
                                        max-length="25"
                                    />
                                    <div class="hint">
                                        "Minimum 8 characters. Include at least one of each: lowercase, uppercase, number, and special characters !@#$%^&*"
                                    </div>
                                    <div class="error" inner_html=error_items></div>
                                </div>
                            }
                                .into_view(cx)
                        } else {
                            view! { cx, <div></div> }
                                .into_view(cx)
                        }
                    }}
                    {move || {
                        if signup_action.pending().get() {
                            view! { cx,
                                <div class="text-center mt-8">
                                    <button class="w-40" disabled>
                                        "Submitting..."
                                    </button>
                                </div>
                            }
                        } else {
                            view! { cx,
                                <div class="flex flex-row text-center justify-between mt-8">
                                    <button class="w-40" type="submit">
                                        "Signup"
                                    </button>
                                    <button class="w-40 red" type="reset">
                                        "Reset"
                                    </button>
                                </div>
                            }
                        }
                    }}
                </div>
            </form>
        </div>
    }
}
