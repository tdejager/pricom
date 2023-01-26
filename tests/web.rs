#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::wasm_bindgen_test_configure;

#[cfg(target_arch = "wasm32")]
wasm_bindgen_test_configure!(run_in_browser);

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test]
fn test_svg_creation() {
    let svg = pricon::generate("me/hello", None).unwrap();
    let children = svg.children();
    let circle = children.get_with_index(0).unwrap();
    assert_eq!(circle.node_name(), "circle");
    let rect = children.get_with_index(1).unwrap();
    assert_eq!(rect.node_name(), "rect");
}