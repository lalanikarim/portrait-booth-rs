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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum ActiveView {
    Login,
    LoginOtp,
    Signup,
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
    let (active_view, set_active_view) = create_signal(cx, ActiveView::Login);
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
            {move || {
                let response = home_page_resource.read(cx);
                match response {
                    None => {
                        view! { cx, <div>"Loading..."</div> }
                            .into_view(cx)
                    }
                    Some(response) => {
                        match response {
                            Err(e) => {
                                view! { cx, <div class="error">"Error: " {e.to_string()}</div> }
                                    .into_view(cx)
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
                                    <div class="container">
                                        <div class="flex flex-row justify-between">
                                            <div style:display=move || if active_view.get() == ActiveView::Login { "none" } else { "inline-block" }>
                                                <button
                                                    on:click=move |_| {
                                                        set_active_view.update(|v| *v = ActiveView::Login);
                                                    }
                                                    class="green w-full"
                                                >
                                                    "Login with Password"
                                                </button>
                                            </div>
                                            <div style:display=move || if active_view.get() == ActiveView::LoginOtp { "none" } else { "inline-block" }>
                                                <button
                                                    on:click=move |_| {
                                                        set_active_view.update(|v| *v = ActiveView::LoginOtp);
                                                    }
                                                    class="green w-full"
                                                >
                                                    "Login with Code"
                                                </button>
                                            </div>
                                            <div style:display=move || if active_view.get() == ActiveView::Signup { "none" } else { "inline-block" }>
                                                <button
                                                    on:click=move |_| {
                                                        set_active_view.update(|v| *v = ActiveView::Signup);
                                                    }
                                                    class="green w-full"
                                                >
                                                    "Signup"
                                                </button>
                                            </div>
                                        </div>
                                    </div>
                                    {move || match active_view.get() {
                                        ActiveView::Login => {
                                            view! { cx, <Login completed/> }
                                        }
                                        ActiveView::LoginOtp => {
                                            view! { cx, <LoginOtp completed/> }
                                        }
                                        ActiveView::Signup => {
                                            view! { cx, <Signup completed/> }
                                        }
                                    }}
                                }
                                    .into_view(cx)
                            }
                        }
                    }
                }
            }}
        </Suspense>
    }
}
