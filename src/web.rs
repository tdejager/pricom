use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::SvgOptions;

// Macro to create a string reference out of something
macro_rules! to_str {
    ($var:expr) => {
        &format!("{}", $var)
    };
}

#[wasm_bindgen]
pub fn generate(
    name: &str,
    options: Option<SvgOptions>
) -> Result<web_sys::Element, JsValue> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    let data = crate::generate_data(name, options.unwrap_or_default());

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    document.body().expect("document should have a body");

    let svg = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "svg")?;
    svg.set_attribute("width", to_str!(data.viewbox.0))?;
    svg.set_attribute("height", to_str!(data.viewbox.1))?;
    svg.set_attribute("viewBox", &format!("0 0 {} {}", data.viewbox.0, data.viewbox.1))?;

    let rotation = format!("rotate({} {} {})", data.rect.rotation, data.circle.x, data.circle.y);
    let circle_fill = format!("hsl({} 80% 80%)", data.circle.hue);
    let rect_fill = format!("hsl({} 80% 30%)", data.rect.hue);
    let stroke = format!("hsl({} 80% 30%)", data.rect.stroke_hue);

    let circle = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "circle")?;
        circle.set_attribute("cx", to_str!(data.circle.x))?;
        circle.set_attribute("cy", to_str!(data.circle.y))?;
        circle.set_attribute("r", to_str!(data.circle.radius))?;
        circle.set_attribute("fill", circle_fill.as_str())?;
    let rectangle = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "rect")?;
        rectangle.set_attribute("width", to_str!(data.rect.size))?;
        rectangle.set_attribute("height", to_str!(data.rect.size))?;
        rectangle.set_attribute("fill", to_str!(rect_fill))?;
        rectangle.set_attribute("transform", to_str!(rotation))?;
        rectangle.set_attribute("stroke", stroke.as_str())?;
        rectangle.set_attribute("stroke-width", to_str!(data.rect.stroke_width))?;
        rectangle.set_attribute("x", to_str!(data.rect.x))?;
        rectangle.set_attribute("y", to_str!(data.rect.y))?;

    svg.append_child(&circle)?;
    svg.append_child(&rectangle)?;
    Ok(svg)
}
