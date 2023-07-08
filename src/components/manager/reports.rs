use leptos::*;

use crate::{
    components::util::{loading::Loading, show_error::ShowError},
    models::report::OrderCountByStatus,
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
    match crate::pool(cx) {
        Err(e) => Err(e),
        Ok(pool) => Report::get_order_count_by_status(&pool).await,
    }
}

#[component]
pub fn Reports(cx: Scope) -> impl IntoView {
    let report = create_resource(
        cx,
        || (),
        move |_| async move { get_order_count_by_status_report(cx).await },
    );
    view! { cx,
        <div class="container">
            <h2 class="header">"Reports"</h2>
            {move || match report.read(cx) {
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
                                                    <td class="border border-slate-300">{format!("{:?}", report_item.status)}</td>
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
        </div>
    }
}