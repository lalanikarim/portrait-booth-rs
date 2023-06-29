use leptos::*;
use serde::{Deserialize, Serialize};

use crate::{
    components::{
        app::AuthUser, auth::login::Login, auth::login_otp::LoginOtp, auth::logout::Logout,
        auth::signup::Signup, orders::orders_view::OrdersView, search::search_view::SearchView,
        util::loading::Loading, util::view_selector::ViewSelector,
    },
    models::user::{Role, User},
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum HomePageViews {
    MyOrders,
    SearchOrders,
    ProcessOrders,
}

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    let set_auth_user =
        use_context::<WriteSignal<AuthUser>>(cx).expect("Set Auth User should be present");
    let (show_view, set_show_view) = create_signal(cx, HomePageViews::MyOrders);
    let completed = create_action(cx, |()| async {});
    let (active_view, set_active_view) = create_signal(cx, ActiveView::Login);
    let home_page_resource = create_resource(
        cx,
        move || (completed.version().get(),),
        move |_| async move { home_page_request(cx).await },
    );
    provide_context::<WriteSignal<HomePageViews>>(cx, set_show_view);
    view! { cx,
        <h1 class="p6 text-4xl">"Portrait Booth"</h1>
        <Transition fallback=move || {
            view! { cx, <Loading/> }
        }>
            {move || {
                let response = home_page_resource.read(cx);
                match response {
                    None => {
                        view! { cx, <Loading/> }
                            .into_view(cx)
                    }
                    Some(response) => {
                        match response {
                            Err(e) => {
                                view! { cx, <div class="error">"Error: " {e.to_string()}</div> }
                                    .into_view(cx)
                            }
                            Ok(HomePageResponse::LoggedIn(user)) => {
                                set_auth_user.set(Some(user.clone()));
                                let user_name = if user.role == Role::Customer
                                    || user.role == Role::Anonymous
                                {
                                    user.name
                                } else {
                                    format!("{:?} ({:?})", user.name, user.role)
                                };
                                let home_page_view = match show_view.get() {
                                    HomePageViews::MyOrders => {
                                        view! { cx, <OrdersView/> }
                                    }
                                    HomePageViews::SearchOrders => {
                                        view! { cx, <SearchView/> }
                                    }
                                    HomePageViews::ProcessOrders => todo!(),
                                };
                                view! { cx,
                                    <div>"Logged in: " {user_name}</div>
                                    <div class="px-6 pt-2 mx-auto max-w-md flex flex-row justify-evenly">
                                        <ViewSelector/>
                                        <Logout completed=completed/>
                                    </div>
                                    {home_page_view}
                                }
                                    .into_view(cx)
                            }
                            Ok(HomePageResponse::NotLoggedIn) => {
                                set_auth_user.set(None);
                                view! { cx,
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
                                            view! { cx, <Signup otp_on_success=true ask_password=false/> }
                                        }
                                    }}
                                }
                                    .into_view(cx)
                            }
                        }
                    }
                }
            }}
        </Transition>
    }
}
