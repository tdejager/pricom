use wasm_bindgen::prelude::wasm_bindgen;
use crate::SvgOptions;

#[wasm_bindgen]
pub fn generate(
    name: &str,
    options: Option<SvgOptions>
) {
    crate::generate_data(name, options.unwrap_or_default());
}
