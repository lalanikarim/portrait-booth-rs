use leptos::{ev::SubmitEvent, html::Input, *};
use leptos_router::*;

#[server(LoginOtpRequest, "/api")]
pub async fn login_otp_request(cx: Scope, email: String) -> Result<(), ServerFnError> {
    use totp_rs::*;
    let pool = crate::pool(cx).expect("Pool should be present");
    let result = sqlx::query_scalar!("SELECT otp_secret FROM users WHERE email = ?", email)
        .fetch_one(&pool)
        .await;
    if let Ok(Some(otp_secret)) = result {
        let totp_dur = crate::get_totp_duration();
        let totp = TOTP::new(
            Algorithm::SHA256,
            6,
            1,
            totp_dur,
            otp_secret.as_bytes().into(),
        )
        .expect("Unable to Initialize TOTP");
        log!(
            "OTP Code: {:#?}, ttl: {:#?}s",
            totp.generate_current().expect("Unable to generate OTP"),
            totp.ttl()
        );
    }
    Ok(())
}
#[server(LoginOtpVerifyRequest, "/api")]
pub async fn login_otp_verify_request(
    cx: Scope,
    email: String,
    otp: String,
) -> Result<bool, ServerFnError> {
    use totp_rs::*;
    let pool = crate::pool(cx).expect("Pool should be present");
    let auth = crate::auth::auth(cx).expect("Auth should be present");
    let result =
        sqlx::query_as::<_, crate::models::user::User>("SELECT * FROM users WHERE email = ?")
            .bind(email)
            .fetch_one(&pool)
            .await;
    if let Ok(user) = result {
        let secret = user.clone().otp_secret.unwrap_or_default();
        let secret = secret.as_bytes();
        let totp_dur = crate::get_totp_duration();
        let totp = TOTP::new(Algorithm::SHA256, 6, 1, totp_dur, secret.into())
            .expect("Unable to Initialize TOTP");
        if totp.check_current(otp.as_str()).ok().unwrap_or_default() {
            auth.logout_user();
            auth.login_user(user.id);
            return Ok(true);
        }
    }
    Ok(false)
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum LoginOtpState {
    GetEmail,
    GetOtp,
}

#[component]
pub fn LoginOtp(cx: Scope, #[prop(optional)] completed: Option<Action<(), ()>>) -> impl IntoView {
    let (state, set_state) = create_signal(cx, LoginOtpState::GetEmail);
    let username_input = create_node_ref::<Input>(cx);
    let password_input = create_node_ref::<Input>(cx);
    let login_otp_request_action = create_server_action::<LoginOtpRequest>(cx);
    let login_otp_verify_action = create_server_action::<LoginOtpVerifyRequest>(cx);
    let error = move || {
        if let Some(Ok(login_otp_response)) = login_otp_verify_action.value().get() {
            match login_otp_response {
                true => "",
                false => "Invalid Code",
            };
        }
    };
    create_effect(cx, move |_| {
        if let Some(Ok(login_otp_response)) = login_otp_verify_action.value().get() {
            match login_otp_response {
                true => {
                    let navigate = use_navigate(cx);
                    _ = navigate("/", Default::default());
                    if let Some(completed) = completed {
                        completed.dispatch(());
                    }
                }
                false => {}
            };
        }
    });
    let disable_controls =
        move || login_otp_request_action.pending().get() || login_otp_verify_action.pending().get();
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let email = username_input
            .get()
            .expect("Username field should be present")
            .value();
        match state.get() {
            LoginOtpState::GetEmail => {
                login_otp_request_action.dispatch(LoginOtpRequest { email });
                set_state.update(|state| *state = LoginOtpState::GetOtp);
            }
            LoginOtpState::GetOtp => {
                let otp = password_input
                    .get()
                    .expect("otp field should be present")
                    .value();
                login_otp_verify_action.dispatch(LoginOtpVerifyRequest { email, otp })
            }
        }
    };
    view! { cx,
        <div class="container">
            <h2 class="header">"Login with OTP"</h2>
            <div class="flex flex-col text-left">
                <form on:submit=on_submit>
                    <div class="flex flex-col">
                        <label for="username_otp">"Email"</label>
                        <input
                            id="username_otp"
                            type="text"
                            disabled=disable_controls
                            node_ref=username_input
                            max-length="25"
                        />
                    </div>
                    {move || {
                        if state.get() == LoginOtpState::GetOtp {
                            view! { cx,
                                <div class="flex flex-col mt-2">
                                    <label for="otp_code">"OTP Code"</label>
                                    <input
                                        id="otp_code"
                                        type="text"
                                        disabled=disable_controls
                                        node_ref=password_input
                                        max-length="25"
                                    />
                                    <span class="error">{error}</span>
                                </div>
                            }
                                .into_view(cx)
                        } else {
                            view! { cx, <div></div> }
                                .into_view(cx)
                        }
                    }}
                    <div class="text-center mt-8">
                        <button class="w-40" type="submit" disabled=disable_controls>
                            {move || match state.get() {
                                LoginOtpState::GetEmail => "Request OTP",
                                LoginOtpState::GetOtp => "Verify OTP",
                            }}
                        </button>
                    </div>
                </form>
            </div>
        </div>
    }
}
