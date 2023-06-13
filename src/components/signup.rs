use crate::{models::user::Role, validate_password};
use leptos::{ev::SubmitEvent, html::Input, *};
use leptos_router::use_navigate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum SignupResponse {
    Success,
    UserNameUnavailable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignupForm {
    pub username: String,
    pub password: String,
    pub fullname: String,
}

#[server(SignupRequest, "/api")]
pub async fn signup_request(cx: Scope, form: SignupForm) -> Result<SignupResponse, ServerFnError> {
    use crate::pool;
    let pool = pool(cx)?;
    match sqlx::query_scalar("SELECT COUNT(1) FROM `users` where trim(lower(username)) = ?")
        .bind(&form.username)
        .fetch_one(&pool)
        .await
    {
        Ok(1) => {
            return Ok(SignupResponse::UserNameUnavailable);
        }
        Err(e) => {
            return Err(ServerFnError::ServerError(e.to_string()));
        }
        _ => (),
    };
    let password_hash = bcrypt::hash(form.password.clone(), 12).unwrap();

    sqlx::query!(
        "INSERT INTO users (name, username, password_hash, role) values (?, ?, ?, ?)",
        form.fullname,
        form.username,
        password_hash,
        Role::Anonymous
    )
    .execute(&pool)
    .await
    .map(|_| SignupResponse::Success)
    .map_err(|e| ServerFnError::ServerError(e.to_string()))
}
#[component]
pub fn signup(cx: Scope) -> impl IntoView {
    let (errors, set_errors) = create_signal::<Vec<String>>(cx, Vec::new());
    let fullname_input = create_node_ref::<Input>(cx);
    let username_input = create_node_ref::<Input>(cx);
    let password_input = create_node_ref::<Input>(cx);
    let confirm_password_input = create_node_ref::<Input>(cx);
    let error_items = move || errors.get().join(" ");
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        set_errors.update(|err| err.clear());
        let fullname = fullname_input
            .get()
            .expect("fullname input should exist")
            .value();
        let username = username_input
            .get()
            .expect("username input should exist")
            .value();
        let password = password_input
            .get()
            .expect("password input should exist")
            .value();
        let confirm_password = confirm_password_input
            .get()
            .expect("confirm password input should exist")
            .value();
        if let Err(password_error) = validate_password(password.clone(), confirm_password.clone()) {
            set_errors.update(move |err| err.extend_from_slice(&password_error));
        } else {
            let form = SignupForm {
                fullname,
                username,
                password,
            };
            spawn_local(async move {
                let navigate = use_navigate(cx);
                match signup_request(cx, form).await {
                    Err(e) => {
                        let err_str = e.to_string();
                        set_errors.update(|err| err.push(err_str));
                    }
                    Ok(result) => {
                        match result {
                            SignupResponse::Success => {
                                _ = navigate("/", Default::default());
                            }
                            SignupResponse::UserNameUnavailable => {
                                set_errors.update(|err| err.push("Username not available".into()));
                            }
                        };
                    }
                }
            });
        }
    };
    view! {
        cx,
        <div class="my-0 mx-auto max-w-sm text-center">
            <h2 class="p-6 text-4xl">"Signup"</h2>
            <form on:submit=on_submit>
                <div class="flex flex-col text-left">
                    <div class="flex flex-col">
                        <label for="fullname">"Full Name"</label>
                        <input id="fullname" type="text" node_ref=fullname_input max-length="25"></input>
                    </div>
                    <div class="flex flex-col">
                        <label for="username">"Username"</label>
                        <input id="username" type="text" node_ref=username_input max-length="25"></input>
                    </div>
                    <div class="flex flex-col mt-2">
                        <label for="password">"Password"</label>
                        <input id="password" type="password" node_ref=password_input max-length="25"></input>
                    </div>
                    <div class="flex flex-col mt-2">
                        <label for="confirm_password">"Confirm Password"</label>
                        <input id="confirm_password" type="password" node_ref=confirm_password_input max-length="25"></input>
                        <div class="hint">"Minimum 8 characters. Include at least one of each: lowercase, uppercase, number, and special characters !@#$%^&*"</div>
                        <div class="error">{error_items}</div>
                    </div>
                    <div class="flex flex-row text-center justify-between mt-8">
                        <button class="w-40" type="submit">"Signup"</button>
                        <button class="w-40 red" type="reset">"Reset"</button>
                    </div>
                </div>
            </form>
        </div>
    }
}
