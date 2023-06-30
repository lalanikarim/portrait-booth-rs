use leptos::*;
use web_sys::MouseEvent;

use crate::{
    components::{home_page::HomePageViews, util::empty_view::EmptyView},
    models::user::{Role, User},
};

#[component]
pub fn ViewSelector(cx: Scope, user: User) -> impl IntoView {
    let create_views = move |show_views: Vec<(HomePageViews, &'static str)>| {
        show_views
            .iter()
            .map(|(show_view, label)| view! {cx,<ViewButton show_view=*show_view label />})
            .collect_view(cx)
    };
    let common_views = vec![
        (HomePageViews::SearchOrders, "Search Orders"),
        (HomePageViews::MyOrders, "My Orders"),
    ];
    let manager_views = common_views.clone();
    let cashier_views = common_views.clone();
    let operator_views = common_views;
    let processor_views = vec![(HomePageViews::ProcessOrders, "Process Orders")];
    match user.role {
        Role::Manager => create_views(manager_views),
        Role::Cashier => create_views(cashier_views),
        Role::Operator => create_views(operator_views),
        Role::Processor => create_views(processor_views),
        _ => view! {cx, <EmptyView /> },
    }
}

#[component]
pub fn ViewButton(cx: Scope, show_view: HomePageViews, label: &'static str) -> impl IntoView {
    let set_show_view =
        use_context::<WriteSignal<HomePageViews>>(cx).expect("Show View Not initialized");
    let on_click = move |_: MouseEvent| {
        set_show_view.set(show_view);
    };
    view! {
        cx,
        <button on:click=on_click>{label}</button>
    }
}
