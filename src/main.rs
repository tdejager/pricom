use sha2::Sha256;
use svg::node::element::{Circle, Rectangle};
use svg::Document;
use sha2::Digest;

/// Rotation
fn rotation(hash: &[u8]) -> u8 {
    hash[0]
}

fn hue(hash: &[u8]) -> u16 {
    let first = hash[0] as u16;
    let second = hash[1] as u16;
    first | (second << 8)
}

fn rect(hash: &[u8]) -> u8 {
    5 + (hash[0] % 45) as u8
}



fn main() {

    let name = "wolfv/mamba";
    let mut hasher = Sha256::new();
    hasher.update(name.as_bytes());
    let result = hasher.finalize();

    let rect_size = rect(&result);
    let circle_pos = 50;
    let rect_pos = circle_pos - rect_size / 2;
    let circle_r = 50;
    let rotation = format!("rotate({} {} {})", rotation(&result[1..]), circle_pos, circle_pos);
    let circle_fill = format!("hsl({} 80% 80%)", hue(&result[2..]));
    let rect_fill = format!("hsl({} 80% 30%)", hue(&result[4..]));
    let stroke = format!("hsl({} 80% 30%)", hue(&result[6..]));

    let circle = Circle::new()
        .set("cx", circle_pos)
        .set("cy",circle_pos)
        .set("r", circle_r)
        .set("fill", circle_fill);
    let rect = Rectangle::new()
        .set("width", rect_size)
        .set("height", rect_size)
        .set("fill", rect_fill)
        .set("transform", rotation)
        .set("stroke", stroke)
        .set("stroke-width", 3)
        .set("x", rect_pos)
        .set("y", rect_pos);

    let document = Document::new()
        .set("viewBox", (0, 0, 100, 100))
        .add(circle)
        .add(rect);

    svg::save("image.svg", &document).unwrap();
}
