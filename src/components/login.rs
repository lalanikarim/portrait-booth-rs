use leptos::{ev::SubmitEvent, html::Input, *};
use leptos_router::use_navigate;
use serde::{Deserialize, Serialize};

use crate::models::user::User;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    use crate::models::user::UserStatus;
    let auth = crate::auth::auth(cx).expect("Auth must be present");
    let pool = crate::pool(cx).expect("MySQL pool must be present");
    let response = match User::get_by_username(username, &pool).await.ok() {
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
pub fn Login(cx: Scope, #[prop(optional)] completed: Option<Action<(), ()>>) -> impl IntoView {
    let username_input: NodeRef<Input> = create_node_ref(cx);
    let password_input: NodeRef<Input> = create_node_ref(cx);
    let login_request_action = create_server_action::<LoginRequest>(cx);
    let error = move || match login_request_action.value().get() {
        Some(response) => match response {
            Ok(response) => match response {
                LoginResponse::InvalidCredentials => "Invalid Credentials".to_string(),
                LoginResponse::NotActivated => "Account not activated yet".to_string(),
                LoginResponse::LockedOut => "Account is locked".to_string(),
                _ => "".to_string(),
            },
            Err(e) => e.to_string(),
        },
        None => "".to_string(),
    };
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
        login_request_action.dispatch(LoginRequest { username, password });
    };
    let login_button_text = move || {
        if login_request_action.pending().get() {
            "Logging in"
        } else {
            "Login"
        }
    };
    let disable_control = move || login_request_action.pending().get();
    create_effect(cx, move |_| {
        let Some(Ok(LoginResponse::LoggedIn(_))) = login_request_action.value().get() else {
            return;
        };
        let navigate = use_navigate(cx);
        _ = navigate("/", Default::default());
        if let Some(completed) = completed {
            completed.dispatch(());
        }
    });
    view! { cx,
        <div class="container">
            <h2 class="header">"Login"</h2>
            <form on:submit=on_submit>
                <div class="flex flex-col text-left">
                    <div class="flex flex-col">
                        <label for="username">"Email"</label>
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
                    </div>
                <div class="error" inner_html=error></div>
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
