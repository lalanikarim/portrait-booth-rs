use leptos::*;
use leptos_router::use_navigate;

use crate::components::app::AuthUser;

#[server(LogoutRequest, "/api")]
pub async fn logout_request(cx: Scope) -> Result<(), ServerFnError> {
    crate::auth::auth(cx).map(|auth| {
        auth.logout_user();
    })
}

#[component]
pub fn Logout(cx: Scope, #[prop(optional)] completed: Option<Action<(), ()>>) -> impl IntoView {
    let set_auth_user =
        use_context::<WriteSignal<AuthUser>>(cx).expect("Set Auth User should be present");
    let on_click = move |_| {
        spawn_local(async move {
            if logout_request(cx).await.is_ok() {
                set_auth_user.set(None);
                let navigate = use_navigate(cx);
                _ = navigate("/", Default::default());
                if let Some(completed) = completed {
                    completed.dispatch(());
                }
            }
        });
    };
    view! { cx,
        <button class="red" on:click=on_click>
            "Logout"
        </button>
    }
}
