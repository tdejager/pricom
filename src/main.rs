#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let name = "wolfv/mamba";
    let svg = pricon::generate(name, pricon::SvgOptions::default());
    svg::save("image.svg", &svg).unwrap();
}


#[cfg(target_arch = "wasm32")]
fn main() {
}
