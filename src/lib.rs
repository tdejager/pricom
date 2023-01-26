use sha2::Sha256;
use sha2::Digest;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(not(target_arch = "wasm32"))]
pub use native::generate;

#[cfg(target_arch = "wasm32")]
mod web;
#[cfg(target_arch = "wasm32")]
pub use web::generate;

// Set different allocator for wasm
#[cfg(target_arch = "wasm32")]
use lol_alloc::{AssumeSingleThreaded, FreeListAllocator};

// SAFETY: This application is single threaded, so using AssumeSingleThreaded is allowed.
#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOCATOR: AssumeSingleThreaded<FreeListAllocator> =
    unsafe { AssumeSingleThreaded::new(FreeListAllocator::new()) };

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
/// clamp from 20 to 45 pixels
fn generate_rect_size(hash: &[u8]) -> u8 {
    20 + (hash[0] % 45) as u8
}

/// Options to pass for Svg generation
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct SvgOptions {
    width: u32,
    height: u32,
    outer_rect_stroke_width: u32,
}


#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
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


pub struct CircleOutput {
    pub x: u32,
    pub y: u32,
    pub radius: u32,
    pub hue: u16,
}

pub struct RectOutput {
    pub x: u32,
    pub y: u32,
    pub rotation: u8,
    pub size: u32,
    pub stroke_width: u32,
    pub hue: u16,
    pub stroke_hue: u16,
}

pub struct SVGData {
    pub circle: CircleOutput,
    pub rect: RectOutput,
    pub viewbox: (u32, u32)
}


/// Generate the Pricom image
fn generate_data<S: Into<String>>(input: S, options: SvgOptions) -> SVGData {

    // Hash the string
    let mut hasher = Sha256::new();
    hasher.update(input.into().as_bytes());
    let result = hasher.finalize();
    let rect_size = generate_rect_size(&result) as u32;

    let circle = CircleOutput {
        x: options.width / 2,
        y: options.height / 2,
        radius: options.width / 2,
        hue: generate_hue(&result[2..]),
    };

    let rect = RectOutput {
        x: circle.x - rect_size / 2,
        y: circle.y - rect_size / 2,
        rotation: generate_rotation_angle(&result[1..]),
        size: rect_size,
        stroke_width: options.outer_rect_stroke_width,
        hue: generate_hue(&result[4..]),
        stroke_hue: generate_hue(&result[6..])
    };

    SVGData {
        circle,
        rect,
        viewbox: (options.width, options.height)
    }
}