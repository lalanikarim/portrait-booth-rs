use leptos::*;
use leptos_router::use_navigate;
use web_sys::MouseEvent;

#[component]
pub fn NotAuthorized(cx: Scope) -> impl IntoView {
    let on_click = move |_: MouseEvent| {
        let navigate = use_navigate(cx);
        _ = navigate("/", Default::default());
    };
    view! {
        cx,
        <div>
        <div class="red">"Not Authorized"</div>
        <button on:click=on_click>"Home"</button>
        </div>
    }
}
