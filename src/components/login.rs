use leptos::{ev::SubmitEvent, html::Input, *};
use leptos_router::use_navigate;
use serde::{Deserialize, Serialize};

use crate::models::user::{User, UserStatus};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub enum LoginResponse {
    LoggedIn(User),
    InvalidCredentials,
    LockedOut,
    NotActivated,
}

#[server(LoginRequest, "/api")]
async fn login_request(
    cx: Scope,
    username: String,
    password: String,
) -> Result<LoginResponse, ServerFnError> {
    let auth = crate::auth::auth(cx).expect("Auth must be present");
    let pool = crate::pool(cx).expect("MySQL pool must be present");
    let response = match User::get_by_username(username, &pool).await {
        None => LoginResponse::InvalidCredentials,
        Some(user) => {
            if let Some(true) = bcrypt::verify(
                password,
                &user.clone().password_hash.unwrap_or("".to_string()),
            )
            .ok()
            {
                match user.status {
                    UserStatus::Disabled => LoginResponse::LockedOut,
                    UserStatus::NotActivatedYet => LoginResponse::NotActivated,
                    UserStatus::Active => {
                        auth.login_user(user.id);
                        LoginResponse::LoggedIn(user)
                    }
                }
            } else {
                LoginResponse::InvalidCredentials
            }
        }
    };
    Ok(response)
}

#[component]
pub fn Login(cx: Scope) -> impl IntoView {
    let (error, set_error) = create_signal(cx, "");
    let username_input: NodeRef<Input> = create_node_ref(cx);
    let password_input: NodeRef<Input> = create_node_ref(cx);
    let login_action_fn = move |LoginForm { username, password }: &LoginForm| {
        let username = username.clone();
        let password = password.clone();
        async move {
            let Ok(response) = login_request(cx, username.clone(), password.clone()).await else {
                panic!("Error encountered")
            };
            let login_err = match response {
                LoginResponse::LoggedIn(_) => {
                    log!("Login successful");
                    let navigate = use_navigate(cx);
                    _ = navigate("/", Default::default());
                    ""
                }
                LoginResponse::InvalidCredentials => {
                    log!("Invalid Credentials");
                    "Invalid Credentials"
                }
                LoginResponse::NotActivated => {
                    log!("Account not activated yet");
                    "Account not activated yet"
                }
                LoginResponse::LockedOut => {
                    log!("Account is locked");
                    "Account is locked"
                }
            };
            set_error.update(|err| *err = login_err);
        }
    };
    let login_action = create_action(cx, login_action_fn);
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let username = username_input
            .get()
            .expect("Username element should be present")
            .value();
        let password = password_input
            .get()
            .expect("Password element should be present")
            .value();
        login_action.dispatch(LoginForm { username, password });
    };
    let login_button_text = move || {
        if login_action.pending().get() {
            "Logging in"
        } else {
            "Login"
        }
    };
    let disable_control = move || login_action.pending().get();
    view! { cx,
        <div class="my-0 mx-auto max-w-sm text-center">
            <h2 class="p-6 text-4xl">"Login"</h2>
            <form on:submit=on_submit>
                <div class="flex flex-col text-left">
                    <div class="flex flex-col">
                        <label for="username">"Username (email or phone)"</label>
                        <input
                            id="username"
                            type="text"
                            node_ref=username_input
                            max-length="25"
                            disabled=disable_control
                        />
                    </div>
                    <div class="flex flex-col mt-2">
                        <label for="password">"Password"</label>
                        <input
                            id="password"
                            type="password"
                            node_ref=password_input
                            max-length="25"
                            disabled=disable_control
                        />
                        <span class="error">{error}</span>
                    </div>
                    <div class="flex flex-row text-center justify-between mt-8">
                        <button class="w-40" type="submit" disabled=disable_control>
                            {login_button_text}
                        </button>
                        <a class="disabled">"Forgot password?"</a>
                    </div>
                </div>
            </form>
        </div>
    }
}
