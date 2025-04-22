#[cfg(not(feature = "ssr"))]
mod csr {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen(
        module = "/src/js/highlight.min.js"
    )]
    extern "C" {
        type HighlightOptions;

        #[wasm_bindgen(js_namespace = default1, js_name = highlightAll)]
        pub fn highlight_all();
    }

}

#[cfg(not(feature = "ssr"))]
pub use csr::*;
