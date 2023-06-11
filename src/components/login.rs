use leptos::{ev::SubmitEvent, html::Input, *};
use serde::{Deserialize, Serialize};

use crate::models::user::User;

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
    let response = match username.as_str() {
        "lock" => LoginResponse::LockedOut,
        "inactive" => LoginResponse::NotActivated,
        _ => {
            if username == password {
                LoginResponse::LoggedIn(User::default())
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
        spawn_local(async move {
            let Ok(response) = login_request(cx, username, password).await else {
                panic!("Error encountered")
            };
            let login_err = match response {
                LoginResponse::LoggedIn(_) => {
                    log!("Login successful");
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
        });
    };
    view! {
        cx,
        <div class="my-0 mx-auto max-w-sm text-center">
            <h2 class="p-6 text-4xl">"Login"</h2>
            <form on:submit=on_submit>
                <div class="flex flex-col text-left">
                    <div class="flex flex-col">
                        <label for="username">"Username"</label>
                        <input id="username" type="text" node_ref=username_input max-length="25"></input>
                    </div>
                    <div class="flex flex-col mt-2">
                        <label for="password">"Password"</label>
                        <input id="password" type="password" node_ref=password_input max-length="25"></input>
                        <span class="error">{error}</span>
                    </div>
                    <div class="flex flex-row text-center justify-between mt-8">
                        <button class="w-40" type="submit">"Login"</button>
                        <a class="disabled">"Forgot password?"</a>
                    </div>
                </div>
            </form>
        </div>
    }
}
