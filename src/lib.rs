use svg::Document;
use image::{imageops::{FilterType, resize}};

mod colors;
use colors::get_colors_from_vec;

mod circles;
use circles::{calculate_png_data, calculate_svg_data};

const SCALING_FACTOR: u8 = 8;
const FILTER_TYPE: FilterType = FilterType::CatmullRom;

/// generate png data in u8 vector format, from id as u8 vector
pub fn generate_png (into_id: &[u8], halfsize_in_pixels: i32) -> Result<Vec<u8>, png::EncodingError> {
    let colors = get_colors_from_vec(into_id);
    let data = calculate_png_data (halfsize_in_pixels, colors);
    make_png_from_data(&data, halfsize_in_pixels)
}

pub fn generate_png_scaled_default (into_id: &[u8], halfsize_in_pixels: i32) -> Result<Vec<u8>, IdenticonError> {
    generate_png_scaled_custom(into_id, halfsize_in_pixels, SCALING_FACTOR, FILTER_TYPE)
}

pub fn generate_png_scaled_custom (into_id: &[u8], halfsize_in_pixels: i32, scaling_factor: u8, filter_type: FilterType) -> Result<Vec<u8>, IdenticonError> {
    let data_large = generate_png(into_id, halfsize_in_pixels*(scaling_factor as i32)).map_err(IdenticonError::Png)?;
    let image_large = image::load_from_memory(&data_large).map_err(IdenticonError::Image)?;
    let image_small = resize(&image_large, (halfsize_in_pixels*2+1) as u32, (halfsize_in_pixels*2+1) as u32, filter_type);
    make_png_from_data(&image_small.to_vec(), halfsize_in_pixels).map_err(IdenticonError::Png)
}

fn make_png_from_data (data: &[u8], halfsize_in_pixels: i32) -> Result<Vec<u8>, png::EncodingError> {
    let mut out: Vec<u8> = Vec::new();
    let mut encoder = png::Encoder::new(&mut out, (halfsize_in_pixels*2+1) as u32, (halfsize_in_pixels*2+1) as u32);
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;
    writer.write_image_data(data)?;
    drop(writer);
    Ok(out)
}

#[derive(Debug)]
pub enum IdenticonError {
    Png(png::EncodingError), // errors from png crate, for png encoder; should not happen, since the png parameters are set properly and tested;
    Image(image::ImageError), // errors from image crate, for loading freshly made png into DynamicImage; also generally should not happen; 
}

impl IdenticonError {
    pub fn show(&self) -> String {
        match &self {
            IdenticonError::Png(e) => format!("Error encoding data into png format: {}", e),
            IdenticonError::Image(e) => format!("Error transforming png data into DynamicImage format: {}", e),
        }
    }
}

impl std::fmt::Display for IdenticonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.show())
    }
}


/// generate svg::Document from id as u8 vector
pub fn generate_svg (into_id: &[u8]) -> Document {
    let unit = 100; // svg is vector format, the unit size is arbitrary, and does not influence the outcome
    let mut document = Document::new()
        .set("viewBox", (-unit, -unit, 2*unit, 2*unit));
    let colors = get_colors_from_vec(into_id);
    let data = calculate_svg_data (unit, colors);
    for x in data.into_iter() {
        document = document.add(x);
    }
    document
}

