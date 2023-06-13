use leptos::*;
use serde::{Deserialize, Serialize};

use crate::{components::logout::Logout, models::user::User};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HomePageResponse {
    LoggedIn(User),
    NotLoggedIn,
}

#[server(HomePageRequest, "/api")]
pub async fn home_page_request(cx: Scope) -> Result<HomePageResponse, ServerFnError> {
    let auth = crate::auth::auth(cx).expect("Auth should be present");
    if let Some(user) = auth.current_user {
        Ok(HomePageResponse::LoggedIn(user))
    } else {
        Ok(HomePageResponse::NotLoggedIn)
    }
}

/// Renders the home page of your application.
#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button

    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    let (loggedin_user, set_loggedin_user) = create_signal(cx, None);

    let user_name = move || {
        loggedin_user
            .get()
            .map_or("".to_string(), |user: User| user.name)
    };

    spawn_local(async move {
        if let Ok(response) = home_page_request(cx).await {
            match response {
                HomePageResponse::NotLoggedIn => {
                    set_loggedin_user.update(|user| *user = None);
                }
                HomePageResponse::LoggedIn(user) => {
                    set_loggedin_user.update(|loggedin_user| *loggedin_user = Some(user));
                }
            }
        } else {
            set_loggedin_user.update(|user| *user = None);
        }
    });

    view! { cx,

        <h1>"Portrait Booth"</h1>
        <button class="bg-red-300 p-2 rounded mx-20 hover:bg-red-600 hover:text-white" on:click=on_click>"Click Me: " {count}</button>
        <div>"Logged in: "{user_name}</div>
        <Logout/>
    }
}
