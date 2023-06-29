use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::orders::confirmation::Confirmation;
use crate::{
    components::{
        auth::login::Login, auth::login_otp::LoginOtp, auth::signup::Signup, home_page::HomePage,
        search::order_search::OrderSearch,
    },
    models::user::User,
};
#[server(GetLoggedInUser, "/api")]
pub async fn get_logged_in_user(cx: Scope) -> Result<Option<User>, ServerFnError> {
    let auth = crate::auth::auth(cx).expect("Auth should be present");
    Ok(auth.current_user)
}

pub type AuthUser = Option<User>;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);
    let (auth_user, set_auth_user) = create_signal::<AuthUser>(cx, None);
    provide_context::<ReadSignal<AuthUser>>(cx, auth_user);
    provide_context::<WriteSignal<AuthUser>>(cx, set_auth_user);

    view! { cx,
        <Stylesheet id="leptos" href="/pkg/portrait-booth.css"/>
        <Title text="Welcome to Leptos"/>
        <Router>
            <main>
                <Routes>
                    <Route
                        path=""
                        view=|cx| {
                            view! { cx,
                                <HomePage/>
                            }
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

                        log!("Email received: {}",email);
                        view!{cx, <LoginOtp email=email show_email />}
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
