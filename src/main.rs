use pricon::SvgOptions;


fn main() {

    let name = "wolfv/mamba";
    let svg = pricon::generate(SvgOptions::default(), name);
    svg::save("image.svg", &svg).unwrap();
}
