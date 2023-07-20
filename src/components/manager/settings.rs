use leptos::*;

use crate::components::{
    orders::orders_view::get_allow_order_creation_setting,
    util::{loading::Loading, show_error::ShowError},
};

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::models::setting::Setting;
    }
}

#[server(ToggleAllowOrderCreation, "/api")]
pub async fn toggle_allow_order_creation(cx: Scope, toggle: bool) -> Result<bool, ServerFnError> {
    let pool = crate::pool(cx)?;
    Setting::toggle_allow_order_creation(toggle, &pool).await
}

#[component]
pub fn Settings(cx: Scope) -> impl IntoView {
    let toggle_allow_order_creation_action = create_server_action::<ToggleAllowOrderCreation>(cx);
    let allow_create_order_setting = create_resource(
        cx,
        move || toggle_allow_order_creation_action.version().get(),
        move |_| get_allow_order_creation_setting(cx),
    );
    view! { cx,
        <div class="container">
            <h2 class="header">"Settings"</h2>
            <div class="flex flex-row justify-between">
                <div class="font-bold">"Allow Order Creation"</div>
                {move || match allow_create_order_setting.read(cx) {
                    None => view! { cx, <Loading/> },
                    Some(Err(e)) => view! { cx, <ShowError error=e.to_string()/> },
                    Some(Ok(is_open)) => {
                        let status = if is_open { "open" } else { "closed" };
                    let toggle = if is_open { "Close" } else { "Open" };
                    let toggle_order_creation = move |_| {
                        toggle_allow_order_creation_action.dispatch(ToggleAllowOrderCreation{toggle:!is_open});
                    };
                        view! { cx, <div>{status}</div>
                            <button on:click=toggle_order_creation>{toggle}</button>
                        }.into_view(cx)
                    }
                }}
            </div>
        </div>
    }
}
