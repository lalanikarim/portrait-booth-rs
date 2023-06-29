use leptos::*;

#[component]
pub fn EmptyView(cx: Scope) -> impl IntoView {
    view! { cx, <div style="position: absolute"></div> }
}
