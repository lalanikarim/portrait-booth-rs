use leptos::{ev::MouseEvent, *};

use crate::get_unit_price;

#[component]
pub fn CreateOrder(cx: Scope) -> impl IntoView {
    let (error, set_error) = create_signal(cx, "");
    let (no_of_pics, set_no_of_pics) = create_signal(cx, 0);
    let (unit_price, set_unit_price) = create_signal(cx, 0);
    let total_price = move || format!("${}", no_of_pics.get() * unit_price.get());
    create_resource(
        cx,
        move || no_of_pics.get(),
        move |_| async move {
            match get_unit_price(cx).await {
                Ok(p) => set_unit_price.update(|price| *price = p),
                Err(_) => set_error.update(|e| *e = "Error fetching unit price"),
            }
        },
    );
    let on_submit = |_| {};
    let no_pics_incr = move |ev: MouseEvent| {
        ev.prevent_default();
        if no_of_pics.get() < 20 {
            set_no_of_pics.update(|n| *n += 1);
        }
    };
    let no_pics_decr = move |ev: MouseEvent| {
        ev.prevent_default();
        if no_of_pics.get() > 0 {
            set_no_of_pics.update(|n| *n -= 1);
        }
    };
    view! { cx,
        <div class="my-0 mx-auto max-w-sm text-center">
            <h2 class="p-6 text-4xl">"Create Order"</h2>
            <form on:submit=on_submit>
                <div class="flex flex-col text-left">
                    <div class="flex flex-col">
                        <label for="no-of-pics">"Number of Pictures"</label>
                        <div class="flex flex-row justify-between">
                            <button class="w-12" on:click=no_pics_decr>"-"</button>
                            <div class="inline-block align-middle">{no_of_pics}</div>
                            <button class="w-12" on:click=no_pics_incr>"+"</button>
                        </div>
                    </div>
                    <div class="flex flex-col mt-2">
                        <label>"Total"</label>
                        <div class="text-right">{total_price}</div>
                        <span class="error">{error}</span>
                    </div>
                    <div class="text-center mt-8">
                        <button class="w-40" type="submit">
                            "Create"
                        </button>
                    </div>
                </div>
            </form>
        </div>
    }
}
