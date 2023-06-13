use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::{home_page::HomePage, login::Login, signup::Signup};

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
                        path="/signup"
                        view=|cx| {
                            view! { cx, <Signup/> }
                        }
                    />
                </Routes>
            </main>
        </Router>
    }
}
