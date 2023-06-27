use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::{
    components::{
        home_page::HomePage,
        login::Login,
        login_otp::LoginOtp,
        orders::{confirmation::Confirmation, order_search::OrderSearch},
        signup::Signup,
    },
    models::user::User,
};

#[server(GetLoggedInUser, "/api")]
pub async fn get_logged_in_user(cx: Scope) -> Result<Option<User>, ServerFnError> {
    let auth = crate::auth::auth(cx).expect("Auth should be present");
    Ok(auth.current_user)
}

pub type AuthUser = Signal<Option<User>>;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);
    let user = create_resource(
        cx,
        || (),
        move |_| async move { get_logged_in_user(cx).await },
    );
    let user = Signal::derive(cx, move || {
        if let Some(Ok(user)) = user.read(cx) {
            user
        } else {
            None
        }
    });
    provide_context::<AuthUser>(cx, user);
    view! { cx,
        <Stylesheet id="leptos" href="/pkg/portrait-booth.css"/>
        <Title text="Welcome to Leptos"/>
        <Router>
            <main>
                <Routes>
                    <Route
                        path=""
                        view=|cx| {
                            view! { cx, <HomePage/> }
                        }
                    />
                    <Route
                        path="/login"
                        view=|cx| {
                            view! { cx, <Login/> }
                        }
                    />
                    <Route
                        path="/otp"
                        view=|cx| {
                        let query = use_query_map(cx);
                        let email = move ||query.with(|query| query.get("email").cloned());
                        let show_email = move ||query.with(|query| query.get("show_email").cloned()).map(|s| s == "true").unwrap_or(false);
                        let Some(email) = email() else {

                            return view! { cx, <LoginOtp  /> }
                        };
                        let show_email = show_email();
                        view!{cx, <LoginOtp email show_email />}
    }
                    />
                    <Route
                        path="/signup"
                        view=|cx| {
                            view! { cx, <Signup otp_on_success=true/> }
                        }
                    />
                    <Route
                        path="/confirmation/:order_ref/:payment_ref"
                        view=|cx| {
                            view! { cx, <Confirmation/> }
                        }
                    />
                    <Route path="/search" view=|cx|{
                                view!{cx, <OrderSearch />}
                            } />
                </Routes>
            </main>
        </Router>
    }
}
