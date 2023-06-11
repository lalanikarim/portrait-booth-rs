use leptos::*;

use crate::components::login::Login;
/// Renders the home page of your application.
#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! { cx,
        <h1>"Portrait Booth"</h1>
        <button class="bg-red-300 p-2 rounded mx-20 hover:bg-red-600 hover:text-white" on:click=on_click>"Click Me: " {count}</button>
        <Login />
    }
}
