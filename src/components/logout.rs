use leptos::*;
use leptos_router::use_navigate;
#[server(LogoutRequest, "/api")]
pub async fn logout_request(cx: Scope) -> Result<(), ServerFnError> {
    let auth = crate::auth::auth(cx).expect("Auth should be present");
    auth.logout_user();
    Ok(())
}

#[component]
pub fn Logout(cx: Scope, #[prop(optional)] completed: Option<Action<(), ()>>) -> impl IntoView {
    let on_click = move |_| {
        spawn_local(async move {
            if let Ok(_) = logout_request(cx).await {
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
