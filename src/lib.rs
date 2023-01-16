use sha2::Sha256;
use sha2::Digest;
use svg::Document;
use svg::node::element::{Circle, Rectangle, SVG};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// Generate a rotation
/// 0 - 256 degrees
fn generate_rotation_angle(hash: &[u8]) -> u8 {
    hash[0]
}

/// Generate a hue
fn generate_hue(hash: &[u8]) -> u16 {
    let first = hash[0] as u16;
    let second = hash[1] as u16;
    first | (second << 8)
}

/// Generate the rectangle size
/// clamp from 5 to 45 pixels
fn generate_rect_size(hash: &[u8]) -> u8 {
    5 + (hash[0] % 45) as u8
}

/// Options to pass for Svg generation
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct SvgOptions {
    width: u32,
    height: u32,
    outer_rect_stroke_width: u32,
}

impl SvgOptions {
    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    pub fn outer_rect_stroke_width(mut self, outer_rect_stroke_width: u32) -> Self {
        self.outer_rect_stroke_width = outer_rect_stroke_width;
        self
    }
}

impl Default for SvgOptions {
    fn default() -> Self {
        Self {
            width: 100,
            height: 100,
            outer_rect_stroke_width: 3,
        }
    }
}


/// Generate the Pricom image
pub fn generate<S: Into<String>>(options: SvgOptions, input: S) -> SVG {

    // Hash the string
    let mut hasher = Sha256::new();
    hasher.update(input.into().as_bytes());
    let result = hasher.finalize();

    // TODO make this generic for non-square sizes
    let rect_size = generate_rect_size(&result) as u32;
    let circle_pos = options.width / 2;
    let circle_r = options.width / 2;
    let rect_pos = circle_pos - rect_size / 2;
    let rotation = format!("rotate({} {} {})", generate_rotation_angle(&result[1..]), circle_pos, circle_pos);
    let circle_fill = format!("hsl({} 80% 80%)", generate_hue(&result[2..]));
    let rect_fill = format!("hsl({} 80% 30%)", generate_hue(&result[4..]));
    let stroke = format!("hsl({} 80% 30%)", generate_hue(&result[6..]));

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
        .set("viewBox", (0, 0, options.width, options.height))
        .add(circle)
        .add(rect);

    document
}




#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn generate_web(name: &str, options: Option<SvgOptions>) {
    crate::generate(options.unwrap_or_default(), name);
}