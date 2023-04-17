use console_error_panic_hook;
use leptos::*;
use log::Level;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    _ = console_log::init_with_level(Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|cx| {
        view! { cx, <App/> }
    });
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        {move || {
            vec![
                view!{ cx, <button class="button-foo button-bar button-baz">"button 1"</button>},
                view!{ cx, <button class="button-foo button-bar button-baz">"button 2"</button>},
            ]
        }}
    }
}
