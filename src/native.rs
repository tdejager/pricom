use svg::Document;
use svg::node::element::{Circle, Rectangle, SVG};
use crate::{generate_data, SvgOptions};

pub fn generate<S: Into<String>>(input: S, options: SvgOptions) -> SVG {
    let data = generate_data(input, options);
    let rotation = format!("rotate({} {} {})", data.rect.rotation, data.circle.x, data.circle.y);
    let circle_fill = format!("hsl({} 80% 80%)", data.circle.hue);
    let rect_fill = format!("hsl({} 80% 30%)", data.rect.hue);
    let stroke = format!("hsl({} 80% 30%)", data.rect.stroke_hue);

    let circle = Circle::new()
        .set("cx", data.circle.x)
        .set("cy",data.circle.y)
        .set("r", data.circle.radius)
        .set("fill", circle_fill);
    let rect = Rectangle::new()
        .set("width", data.rect.size)
        .set("height", data.rect.size)
        .set("fill", rect_fill)
        .set("transform", rotation)
        .set("stroke", stroke)
        .set("stroke-width", data.rect.stroke_width)
        .set("x", data.rect.x)
        .set("y", data.rect.y);

    let document = Document::new()
        .set("viewBox", (0, 0, data.viewbox.0, data.viewbox.1))
        .add(circle)
        .add(rect);

    document
}
