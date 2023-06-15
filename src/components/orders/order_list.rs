use leptos::*;

use crate::components::orders::create_order::CreateOrder;

#[component]
pub fn OrderList(cx: Scope) -> impl IntoView {
    let (_created, set_created) = create_signal(cx, false);
    view! {
        cx,
        <div class="my-0 mx-auto max-w-sm text-center">
            <h2 class="p-6 text-4xl">"List of Orders"</h2>
        </div>
        <CreateOrder set_created />
    }
}
