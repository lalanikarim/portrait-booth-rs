use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::{
    home_page::HomePage, login::Login, login_otp::LoginOtp, orders::confirmation::Confirmation,
    signup::Signup,
};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

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
                            view! { cx, <LoginOtp/> }
                        }
                    />
                    <Route
                        path="/signup"
                        view=|cx| {
                            view! { cx, <Signup/> }
                        }
                    />
                    <Route
                        path="/confirmation/:order_ref/:payment_ref"
                        view=|cx| {
                            view! { cx, <Confirmation/> }
                        }
                    />
                </Routes>
            </main>
        </Router>
    }
}
