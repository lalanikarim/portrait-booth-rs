use leptos::*;
use serde::{Deserialize, Serialize};

use crate::{
    components::{
        login::Login, login_otp::LoginOtp, logout::Logout, orders::orders_view::OrdersView,
        signup::Signup,
    },
    models::user::User,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HomePageResponse {
    LoggedIn(User),
    NotLoggedIn,
}

#[server(HomePageRequest, "/api")]
pub async fn home_page_request(cx: Scope) -> Result<HomePageResponse, ServerFnError> {
    let auth = crate::auth::auth(cx).expect("Auth should be present");
    let response = if let Some(user) = auth.current_user {
        Ok(HomePageResponse::LoggedIn(user))
    } else {
        Ok(HomePageResponse::NotLoggedIn)
    };
    response
}

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    let completed = create_action(cx, |()| async {});
    let home_page_resource = create_resource(
        cx,
        move || (completed.version().get(),),
        move |_| async move { home_page_request(cx).await },
    );
    view! { cx,
        <h1 class="p6 text-4xl">"Portrait Booth"</h1>
        <Suspense fallback=move || {
            view! { cx, <div>"Loading..."</div> }
        }>
            {
                move || {
                    let response = home_page_resource.read(cx);
                    match response {
                        None => {
                            view! { cx, <div>"Loading..."</div> }
                                .into_view(cx)
                        }
                        Some(response) => {
                            match response {
                                Err(e) => {
                                    view!{cx, <div class="error">"Error: "{e.to_string()}</div>}.into_view(cx)
                                }
                                Ok(HomePageResponse::LoggedIn(user)) => {
                                    view! { cx,
                                        <div>"Logged in: " {user.name}</div>
                                        <Logout completed=completed/>
                                        <OrdersView/>
                                    }
                                        .into_view(cx)
                                }
                                Ok(HomePageResponse::NotLoggedIn) => {
                                    view! { cx,
                                        <div>"Not Logged In Response"</div>
                                        <Login completed=completed/>
                                        <LoginOtp completed=completed/>
                                        <Signup completed=completed/>
                                    }
                                        .into_view(cx)
                                }
                            }
                        }
                    }
                }
            }
        </Suspense>
    }
}
