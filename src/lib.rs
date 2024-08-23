pub mod api;
pub mod app;
pub mod consts;
pub mod hljs;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;
    use consts::LEPTOS_HYDRATED;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);

    let window = leptos::prelude::window();
    js_sys::Reflect::set(
        &window,
        &wasm_bindgen::JsValue::from_str(LEPTOS_HYDRATED),
        &wasm_bindgen::JsValue::TRUE,
    )
    .expect("error setting hydrated status");
    let event = web_sys::Event::new(LEPTOS_HYDRATED)
        .expect("error creating hydrated event");
    let document = leptos::prelude::document();
    document
        .dispatch_event(&event)
        .expect("error dispatching hydrated event");
    leptos::logging::log!("dispatched hydrated event");
}
