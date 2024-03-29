use leptos::*;

use crate::{
    components::util::{loading::Loading, show_error::ShowError},
    models::report::{OrderCountByProcessor, OrderCountByStatus, PaymentCollection},
};

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::models::report::Report;
    }
}

#[server(GetOrderCountByStatusReport, "/api")]
pub async fn get_order_count_by_status_report(
    cx: Scope,
) -> Result<Vec<OrderCountByStatus>, ServerFnError> {
    let (pool, user) = crate::server::pool_and_current_user(cx)?;
    if user.role == crate::models::user::Role::Manager {
        Report::get_order_count_by_status(&pool).await
    } else {
        Err(ServerFnError::ServerError(
            "Only Managers can execute".into(),
        ))
    }
}

#[server(GetCollectionReport, "/api")]
pub async fn get_collection_report(cx: Scope) -> Result<Vec<PaymentCollection>, ServerFnError> {
    let (pool, user) = crate::server::pool_and_current_user(cx)?;
    if user.role == crate::models::user::Role::Manager {
        Report::get_collection_by_staff(&pool).await
    } else {
        Err(ServerFnError::ServerError(
            "Only Managers can execute".into(),
        ))
    }
}

#[server(GetProcessorReport, "/api")]
pub async fn get_processor_report(cx: Scope) -> Result<Vec<OrderCountByProcessor>, ServerFnError> {
    let (pool, user) = crate::server::pool_and_current_user(cx)?;

    if user.role == crate::models::user::Role::Manager {
        Report::get_order_count_by_processor(&pool).await
    } else {
        Err(ServerFnError::ServerError(
            "Only Managers can execute".into(),
        ))
    }
}

#[component]
pub fn Reports(cx: Scope) -> impl IntoView {
    let order_counts_report =
        create_resource(cx, || (), move |_| get_order_count_by_status_report(cx));
    let collection_report = create_resource(cx, || (), move |_| get_collection_report(cx));
    let processor_report = create_resource(cx, || (), move |_| get_processor_report(cx));
    view! { cx,
        <div class="container-lg">
            <h2 class="header">"Reports"</h2>
            <div class="text-lg">"Order Counts"</div>
            {move || match order_counts_report.read(cx) {
                None => view! { cx, <Loading/> },
                Some(Err(e)) => view! { cx, <ShowError error=e.to_string()/> },
                Some(Ok(report)) => {
                    if report.is_empty() {
                        view! { cx, <div>"No records found!"</div> }.into_view(cx)
                    } else {
                        view! { cx,
                            <table class="table-auto w-full broder-collapse border border-slate-400">
                                <thead class="bg-slate-50">
                                    <tr>
                                        <th class="border border-slate-300">"Status"</th>
                                        <th class="border border-slate-300">"Count"</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {report
                                        .into_iter()
                                        .map(|report_item| {
                                            view! { cx,
                                                <tr>
                                                    <td class="border border-slate-300">
                                                        {format!("{:?}", report_item.status)}
                                                    </td>
                                                    <td class="border border-slate-300">{report_item.count}</td>
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
                }
            }}
            <div class="text-lg">"Collection Report"</div>
            {move || match collection_report.read(cx) {
                None => view! { cx, <Loading/> },
                Some(Err(e)) => view! { cx, <ShowError error=e.to_string()/> },
                Some(Ok(report)) => {
                    if report.is_empty() {
                        view! { cx, <div>"No records found!"</div> }.into_view(cx)
                    } else {
                        view! { cx,
                            <table class="table-auto w-full broder-collapse border border-slate-400">
                                <thead class="bg-slate-50">
                                    <tr>
                                        <th class="border border-slate-300">"Staff"</th>
                                        <th class="border border-slate-300">"Count"</th>
                                        <th class="border border-slate-300">"Total"</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {report
                                        .into_iter()
                                        .map(|report_item| {
                                            view! { cx,
                                                <tr>
                                                    <td class="border border-slate-300 text-left">
                                                        {format!(
                                                            "{}{}", report_item.name, report_item.email.map(| email |
                                                            format!(" ({email})")).unwrap_or_default()
                                                        )}
                                                    </td>
                                                    <td class="border border-slate-300">{report_item.count}</td>
                                                    <td class="border border-slate-300">
                                                        "$" {report_item.total}
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
                }
            }}
            <div class="text-lg">"Processor Report"</div>
            {move || match processor_report.read(cx) {
                None => view! { cx, <Loading/> },
                Some(Err(e)) => view! { cx, <ShowError error=e.to_string()/> },
                Some(Ok(report)) => {
                    if report.is_empty() {
                        view! { cx, <div>"No records found!"</div> }.into_view(cx)
                    } else {
                        view! { cx,
                            <table class="table-auto w-full broder-collapse border border-slate-400">
                                <thead class="bg-slate-50">
                                    <tr>
                                        <th class="border border-slate-300">"Staff"</th>
                                        <th class="border border-slate-300">"Orders"</th>
                                        <th class="border border-slate-300">"Photos"</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {report
                                        .into_iter()
                                        .map(|report_item| {
                                            view! { cx,
                                                <tr>
                                                    <td class="border border-slate-300 text-left">
                                                        {format!("{} ({})", report_item.name, report_item.email)}
                                                    </td>
                                                    <td class="border border-slate-300">
                                                        {report_item.order_count}
                                                    </td>
                                                    <td class="border border-slate-300">
                                                        {report_item.photos_count}
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
                }
            }}
        </div>
    }
}
