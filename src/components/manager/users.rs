use leptos::{html::Input, *};
use web_sys::MouseEvent;

use crate::{
    components::util::{empty_view::EmptyView, loading::Loading, show_error::ShowError},
    models::user::{Role, User},
};

#[server(GetAllStaff, "/api")]
pub async fn get_all_staff(cx: Scope) -> Result<Vec<User>, ServerFnError> {
    let pool = crate::pool(cx)?;
    User::get_all_staff(&pool).await
}

#[server(GetUserByEmail, "/api")]
pub async fn get_user_by_email(cx: Scope, email: String) -> Result<Option<User>, ServerFnError> {
    let pool = crate::pool(cx)?;
    User::get_by_username(email, &pool).await.map(Some)
}

#[server(ChangeUserRole, "/api")]
pub async fn change_user_role(cx: Scope, id: u64, role: Role) -> Result<bool, ServerFnError> {
    let pool = crate::pool(cx)?;
    User::change_role(id, role, &pool).await
}

#[component]
pub fn Users(cx: Scope) -> impl IntoView {
    let (email, set_email) = create_signal::<Option<String>>(cx, None);
    let change_role_action = create_server_action::<ChangeUserRole>(cx);
    let email_input = create_node_ref::<Input>(cx);
    let all_staff = create_resource(
        cx,
        move || (email.get(), change_role_action.version().get()),
        move |(email, _)| async move {
            match email {
                None => get_all_staff(cx).await.map(Some),
                Some(email) => get_user_by_email(cx, email)
                    .await
                    .map(|user| user.map(|user| vec![user])),
            }
        },
    );
    let search_click = move |_: MouseEvent| {
        let email = email_input.get().expect("Email Input should be present");
        if email.value().len() > 0 {
            set_email.set(Some(email.value()));
        }
    };
    let get_all_click = move |_: MouseEvent| {
        let email = email_input.get().expect("Email Input should be present");
        email.set_value("");
        set_email.set(None);
    };
    view! { cx,
        <div class="container-lg">
            <h2 class="header">"Users"</h2>
            <div class="flex flex-row justify-between">
                <input _ref=email_input placeholder="Enter Email..."/>
                <div>
                    <button on:click=search_click type="submit">"Search"</button>
                    <button on:click=get_all_click>"Get All"</button>
                </div>
            </div>
            {move || match all_staff.read(cx) {
                None => view! { cx, <Loading/> },
                Some(Err(e)) => view! { cx, <ShowError error=e.to_string()/> },
                Some(Ok(None)) => view! { cx, <div>"No records returned"</div> }.into_view(cx),
                Some(Ok(Some(staff))) => {
                    view! { cx,
                        <table class="table-auto w-full broder-collapse border border-slate-400">
                            <thead class="bg-slate-50">
                                <tr>
                                    <th class="border border-slate-300">"Name"</th>
                                    <th class="border border-slate-300">"Email"</th>
                                    <th class="border border-slate-300">"Role"</th>
                                </tr>
                            </thead>
                            <tbody>
                                {staff
                                    .iter()
                                    .map(|user| {
                                        let user = user.clone();
                                        view! { cx,
                                            <tr>
                                                <td class="border border-slate-300">{user.name}</td>
                                                <td class="border border-slate-300">{user.email}</td>
                                                <td class="border border-slate-300">
                                                    {format!("{:?}", user.role)}
                                                </td>
                                            </tr>
                                            <tr>
                                                <td colspan=3 class="border border-slate-300">
                                                    <ChangeRoleButtons
                                                        id=user.id
                                                        role=user.role
                                                        change_role_action
                                                    />
                                                </td>
                                            </tr>
                                        }
                                            .into_view(cx)
                                    })
                                    .collect_view(cx)}
                            </tbody>
                        </table>
                    }
                        .into_view(cx)
                }
            }}
        </div>
    }
}

#[component]
pub fn ChangeRoleButtons(
    cx: Scope,
    id: u64,
    role: Role,
    change_role_action: Action<ChangeUserRole, Result<bool, ServerFnError>>,
) -> impl IntoView {
    view! { cx,
        <div class="flex flex-row justify-around">
            {if role != Role::Customer {
                view! { cx,
                    <button on:click=move |_| {
                        change_role_action
                            .dispatch(ChangeUserRole {
                                id,
                                role: Role::Customer,
                            })
                    }>"Customer"</button>
                }
                    .into_view(cx)
            } else {
                view! { cx, <EmptyView/> }
            }}
            {if role != Role::Cashier {
                view! { cx,
                    <button on:click=move |_| {
                        change_role_action
                            .dispatch(ChangeUserRole {
                                id,
                                role: Role::Cashier,
                            })
                    }>"Cashier"</button>
                }
                    .into_view(cx)
            } else {
                view! { cx, <EmptyView/> }
            }}
            {if role != Role::Operator {
                view! { cx,
                    <button on:click=move |_| {
                        change_role_action
                            .dispatch(ChangeUserRole {
                                id,
                                role: Role::Operator,
                            })
                    }>"Operator"</button>
                }
                    .into_view(cx)
            } else {
                view! { cx, <EmptyView/> }
            }}
            {if role != Role::Processor {
                view! { cx,
                    <button on:click=move |_| {
                        change_role_action
                            .dispatch(ChangeUserRole {
                                id,
                                role: Role::Processor,
                            })
                    }>"Processor"</button>
                }
                    .into_view(cx)
            } else {
                view! { cx, <EmptyView/> }
            }}
            {if role != Role::Manager {
                view! { cx,
                    <button on:click=move |_| {
                        change_role_action
                            .dispatch(ChangeUserRole {
                                id,
                                role: Role::Manager,
                            })
                    }>"Manager"</button>
                }
                    .into_view(cx)
            } else {
                view! { cx, <EmptyView/> }
            }}
        </div>
    }
}
