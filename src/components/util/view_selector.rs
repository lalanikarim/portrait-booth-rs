use leptos::*;

use crate::{
    components::{app::AuthUser, home_page::HomePageViews, util::empty_view::EmptyView},
    models::user::Role,
};

#[component]
pub fn ViewSelector(cx: Scope) -> impl IntoView {
    let auth_user = use_context::<ReadSignal<AuthUser>>(cx).expect("Auth User should exist");
    let set_show_view =
        use_context::<WriteSignal<HomePageViews>>(cx).expect("Set Show View should exist");
    let Some(user) = auth_user.get() else {
        return view!{cx, <EmptyView />}
    };
    let my_orders_button = view! {cx,
        <button on:click=move|_|set_show_view.set(HomePageViews::MyOrders)>"My Orders"</button>
    };
    let search_orders_button = view! {cx,
        <button on:click=move|_|set_show_view.set(HomePageViews::SearchOrders)>"Search Orders"</button>
    };
    let process_orders_button = view! {cx,
        <button on:click=move|_|set_show_view.set(HomePageViews::ProcessOrders)>"Process Orders"</button>
    };

    match user.role {
        Role::Manager => vec![search_orders_button, my_orders_button].collect_view(cx),
        Role::Cashier => vec![search_orders_button, my_orders_button].collect_view(cx),
        Role::Operator => vec![search_orders_button, my_orders_button].collect_view(cx),
        Role::Processor => process_orders_button.into_view(cx),
        _ => view! {cx, <EmptyView /> },
    }
}
