use leptos::*;

#[component]
pub fn ShowError(cx: Scope, error: String) -> impl IntoView {
    view! {cx,
        <div class="red">{error}</div>
    }
}
