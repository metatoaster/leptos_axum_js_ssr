use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(
    module = "/node_modules/@highlightjs/cdn-assets/es/highlight.min.js"
)]
extern "C" {
    #[wasm_bindgen(js_namespace = default, js_name = highlightAll)]
    pub fn highlight_all();
}
