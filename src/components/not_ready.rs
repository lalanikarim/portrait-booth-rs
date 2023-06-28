use leptos::{leptos_dom::helpers::location, *};
use web_sys::MouseEvent;

#[component]
pub fn NotReady(cx: Scope) -> impl IntoView {
    let on_click = move |_: MouseEvent| {
        let location = location();
        _ = location.reload();
    };
    view! {
        cx,
        <div>
        <div class="red">"Not Ready"</div>
        <button on:click=on_click>"Reload"</button>
        </div>
    }
}
