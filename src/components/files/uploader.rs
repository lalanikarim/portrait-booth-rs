use leptos::*;
//use web_sys::DragEvent;

#[component]
pub fn Uploader(cx: Scope) -> impl IntoView {
    let on_drop = move |ev: ev::DragEvent| {
        log!("{ev:#?}");
        ev.prevent_default();
        /*
        let dt = ev.data_transfer().unwrap();
        let files = dt.get_files();
        let promise = match files {
            Ok(promise) => promise,
            Err(_) => return,
        };
        */
    };
    view! { cx,
        <div class="container">
            <h2 class="header">"Upload Files"</h2>
            <div class="m-5 h-60 border-4 border-dashed rounded-xl">
                <div
                    class="flex items-center justify-center h-full"
                    on:dragover=move |ev| {
                        log!("Drag over");
                        ev.prevent_default();
                    }
                    on:drop=on_drop
                >
                    "Test"
                </div>
            </div>
        </div>
    }
}
