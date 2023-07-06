use leptos::*;
use serde::{Deserialize, Serialize};

use crate::{
    components::{
        app::AuthUser,
        auth::login_otp::LoginOtp,
        auth::logout::Logout,
        auth::signup::Signup,
        error_template::ErrorTemplate,
        orders::orders_view::OrdersView,
        search::search_view::SearchView,
        util::view_selector::ViewSelector,
        util::{empty_view::EmptyView, loading::Loading},
    },
    models::{
        pricing::Pricing,
        user::{Role, User},
    },
};

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::models::order::Order;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HomePageResponse {
    LoggedIn(User),
    NotLoggedIn,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum ActiveView {
    LoginOtp,
    Signup,
}

#[server(GetUnitPrice, "/api")]
pub async fn get_unit_price() -> Result<Pricing, ServerFnError> {
    Order::get_unit_price()
}
#[server(HomePageRequest, "/api")]
pub async fn home_page_request(cx: Scope) -> Result<HomePageResponse, ServerFnError> {
    crate::auth::auth(cx).map(|auth| {
        if let Some(user) = auth.current_user {
            HomePageResponse::LoggedIn(user)
        } else {
            HomePageResponse::NotLoggedIn
        }
    })
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, Copy)]
pub enum HomePageViews {
    Loading,
    MyOrders,
    SearchOrders,
    ProcessOrders,
}

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    let set_auth_user =
        use_context::<WriteSignal<AuthUser>>(cx).expect("Set Auth User should be present");
    let (show_view, set_show_view) = create_signal(cx, HomePageViews::Loading);
    let completed = create_action(cx, |()| async {});
    let (active_view, set_active_view) = create_signal(cx, ActiveView::Signup);
    let home_page_resource = create_resource(
        cx,
        move || (completed.version().get(),),
        move |_| async move { home_page_request(cx).await },
    );
    provide_context::<WriteSignal<HomePageViews>>(cx, set_show_view);
    let unit_price_resource =
        create_resource(cx, || (), move |_| async move { get_unit_price().await });
    provide_context(cx, unit_price_resource);
    view! { cx,
        <h1 class="p6 text-4xl">"Portrait Booth"</h1>
        <Transition fallback=move || {
            view! { cx, <Loading/> }
        }>
            <ErrorBoundary fallback=|cx, errors| {
                view! { cx, <ErrorTemplate errors/> }
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
                                    let first_view = match user.role {
                                        Role::Manager => HomePageViews::SearchOrders,
                                        Role::Operator => HomePageViews::SearchOrders,
                                        Role::Cashier => HomePageViews::SearchOrders,
                                        Role::Processor => HomePageViews::ProcessOrders,
                                        _ => HomePageViews::MyOrders,
                                    };
                                    set_show_view.set(first_view);
                                    let user_name = if user.role.clone() == Role::Customer
                                        || user.role.clone() == Role::Anonymous
                                    {
                                        user.name.clone()
                                    } else {
                                        format!("{:?} ({:?})", user.name.clone(), user.role.clone())
                                    };
                                    view! { cx,
                                        <div>"Logged in: " {user_name}</div>
                                        <div class="px-6 pt-2 mx-auto max-w-md flex flex-row justify-evenly">
                                            <ViewSelector user/>
                                            <Logout completed=completed/>
                                        </div>
                                        {move || match show_view.get() {
                                            HomePageViews::MyOrders => {
                                                view! { cx, <OrdersView/> }
                                            }
                                            HomePageViews::SearchOrders => {
                                                view! { cx, <SearchView/> }
                                            }
                                            HomePageViews::Loading => {
                                                view! { cx,
                                                    <div class="container">
                                                        <Loading/>
                                                    </div>
                                                }
                                                    .into_view(cx)
                                            }
                                            _ => {
                                                view! { cx, <EmptyView/> }
                                            }
                                        }}
                                    }
                                        .into_view(cx)
                                }
                                Ok(HomePageResponse::NotLoggedIn) => {
                                    set_auth_user.set(None);
                                    view! { cx,
                                        <div class="container">
                                            <div class="flex flex-row justify-between">
                                                {match active_view.get() {
                                                    ActiveView::Signup => {
                                                        view! { cx,
                                                            <label>"Already signed up?"</label>
                                                            <button
                                                                on:click=move |_| {
                                                                    set_active_view.update(|v| *v = ActiveView::LoginOtp);
                                                                }
                                                                class="green w-40"
                                                            >
                                                                "Login with Code"
                                                            </button>
                                                        }
                                                    }
                                                    ActiveView::LoginOtp => {
                                                        view! { cx,
                                                            <label>"Haven't signed up yet?"</label>
                                                            <button
                                                                on:click=move |_| {
                                                                    set_active_view.update(|v| *v = ActiveView::Signup);
                                                                }
                                                                class="green w-40"
                                                            >
                                                                "Signup"
                                                            </button>
                                                        }
                                                    }
                                                }}
                                            </div>
                                        </div>
                                        {move || match active_view.get() {
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
            </ErrorBoundary>
        </Transition>
    }
}
