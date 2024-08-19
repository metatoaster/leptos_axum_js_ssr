pub mod api;
pub mod app;
pub mod consts;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;
    use consts::LEPTOS_HYDRATED;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);

    let window = web_sys::window().expect("window must exist in this context");
    js_sys::Reflect::set(
        &window,
        &wasm_bindgen::JsValue::from_str(LEPTOS_HYDRATED),
        &wasm_bindgen::JsValue::TRUE,
    )
    .expect("error setting hydrated status");
    let event = web_sys::Event::new(LEPTOS_HYDRATED)
        .expect("error creating hydrated event");
    let document = window.document().expect("document is missing");
    document
        .dispatch_event(&event)
        .expect("error dispatching hydrated event");
}
