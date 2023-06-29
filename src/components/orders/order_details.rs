use leptos::*;

use crate::{
    components::{
        app::AuthUser,
        orders::{
            actions::cashier_actions::CashierActions, actions::customer_actions::CustomerActions,
        },
        util::loading::Loading,
        util::not_authorized::NotAuthorized,
        util::not_ready::NotReady,
    },
    models::{order::OrderStatus, user::Role, user_order::UserOrder},
};

#[component]
pub fn OrderDetails(cx: Scope, order: UserOrder) -> impl IntoView {
    let auth_user = use_context::<ReadSignal<AuthUser>>(cx).expect("AuthUser should be present");
    let Some(user) = auth_user.get() else {
        return view!{cx, <NotReady /> }.into_view(cx);
    };
    if user.id != order.customer_id
        && ![Role::Cashier, Role::Operator, Role::Manager].contains(&user.role)
    {
        return view! { cx, <NotAuthorized /> }.into_view(cx);
    }
    let set_order = use_context::<WriteSignal<Option<UserOrder>>>(cx)
        .expect("Set_order write signal should be present");
    view! { cx,
        <div class="container">
            <h2 class="header">"Order Details"</h2>
            <div class="flex flex-row text-left">
                <div class="w-1/2">"Order #"</div>
                <div class="">{order.id}</div>
            </div>
            <div class="flex flex-row text-left">
                <div class="w-1/2">"No of Photos"</div>
                <div>{order.no_of_photos}</div>
            </div>
            <div class="flex flex-row text-left">
                <div class="w-1/2">"Order total"</div>
                <div>"$" {order.order_total}</div>
            </div>
            <div class="flex flex-row text-left">
                <div class="w-1/2">"Name"</div>
                <div>{order.name.clone()}</div>
            </div>
            <div class="flex flex-row text-left">
                <div class="w-1/2">"Email"</div>
                <div>{order.email.clone()}</div>
            </div>
            <div class="flex flex-row text-left">
                <div class="w-1/2">"Phone"</div>
                <div>{order.phone.clone().unwrap_or("".to_string())}</div>
            </div>
            <div class="flex flex-row text-left">
                <div class="w-1/2">"Status"</div>
                <div>
                    {if order.status == OrderStatus::PaymentPending {
                        format!("{:?} ({:?})", order.status, order.mode_of_payment)
                    } else {
                        format!("{:?}", order.status)
                    }}
                </div>
            </div>
            <button class="m-1" type="button" on:click=move |_| set_order.update(|o| *o = None)>
                "Back"
            </button>
            <Suspense fallback=move || {
                view! { cx, <Loading /> }
            }>
                <CustomerActions order=order.clone()/>
                <CashierActions order=order.clone()/>
            </Suspense>
        </div>
    }
    .into_view(cx)
}
