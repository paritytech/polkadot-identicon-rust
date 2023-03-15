//! Generates 19-circle identicons in `png` and in `svg` format from `&[u8]` input slice. Slice length could be arbitrary, as identicon generation starts with input hashing. Typical input is a public key.  
//!
//! Identicon example:  
#![doc=include_str!("identicon_example.svg")]
//!
//! The identicon color scheme and elements arrangement follow the published javascript [code](https://github.com/paritytech/oo7/blob/master/packages/polkadot-identicon/src/index.jsx) for polkadot identicon generation. This crate is intended mainly for use by [Signer](https://github.com/paritytech/parity-signer).  
//!
//! Crate also supports generation of identicon-like images with pre-set colors in RGBA format, mainly for test purposes.  
//!
//! Feature `"pix"` supports generation of `png` images, feature `"vec"` - generation of `svg` images. Both are made available by default.  

#![deny(unused_crate_dependencies)]

#[cfg(feature = "pix")]
use image::{
    imageops::{resize, FilterType},
    load_from_memory,
};

#[cfg(feature = "vec")]
use svg::Document;

pub mod circles;
pub mod colors;
pub use colors::Color;

#[cfg(feature = "pix")]
const SIZE_IN_PIXELS: u8 = 30;
#[cfg(feature = "pix")]
const SCALING_FACTOR: u8 = 5;
#[cfg(feature = "pix")]
const FILTER_TYPE: FilterType = FilterType::Lanczos3;
#[cfg(feature = "pix")]
/// Static 30x30 transparent `png`, for rare cases when the identicon
/// generation fails or when the data to generate identicon is unavailable
pub const EMPTY_PNG: &[u8] = &[
    137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 30, 0, 0, 0, 30, 8, 6,
    0, 0, 0, 59, 48, 174, 162, 0, 0, 0, 46, 73, 68, 65, 84, 120, 156, 237, 205, 65, 1, 0, 32, 12,
    0, 33, 237, 31, 122, 182, 56, 31, 131, 2, 220, 153, 57, 63, 136, 51, 226, 140, 56, 35, 206,
    136, 51, 226, 140, 56, 35, 206, 136, 51, 251, 226, 7, 36, 207, 89, 197, 10, 134, 29, 92, 0, 0,
    0, 0, 73, 69, 78, 68, 174, 66, 96, 130,
];

/// Polkadot identicon `png` data in `u8` vector format, from `&[u8]` input slice
///
/// Input slice could be of any length, as it gets hashed anyways;
/// typical input is a public key.
///
/// Resulting image quality depends on image size,
/// images start looking acceptable for sizes apporimately 100 pix and above.
///
/// ## Example
///
/// Let's generate a set of identicons of varying size.
///
/// ```
/// use image::load_from_memory;
/// use plot_icon::generate_png;
///
/// let alice: &[u8] = &[212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133, 88, 133, 76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125];
/// let size_set = [16, 32, 45, 64, 128, 256, 512];
/// for size_in_pixels in size_set.into_iter() {
///     let content = generate_png(alice, size_in_pixels).unwrap();
///     let image = load_from_memory(&content).unwrap();
///     assert!(image.width() == size_in_pixels as u32);
///     assert!(image.height() == size_in_pixels as u32);
/// }
/// ```
#[cfg(feature = "pix")]
pub fn generate_png(into_id: &[u8], size_in_pixels: u16) -> Result<Vec<u8>, png::EncodingError> {
    let colors = colors::get_colors(into_id);
    generate_png_with_colors(colors, size_in_pixels)
}

/// Polkadot identicon `png` data in `u8` vector format, with given colors
///
/// Makes regular identicons if properly generated color set is plugged in.
/// Also could be used to generate test identicon-like pictures.
///
/// Input [`Color`] set is in RGBA format.
///
/// ## Example
///
/// Make identicon with gray circles.
/// Such identicon-like image could be used as constant for rare cases when real `png` identicon failed to generate.
///
/// ```
/// use plot_icon::{generate_png_with_colors, colors::Color};
///
/// let colors = [Color{red: 200, green: 200, blue: 200, alpha: 255}; 19];
/// let size_in_pixels = 250;
/// let content = generate_png_with_colors(colors, size_in_pixels).unwrap();
/// ```
#[cfg(feature = "pix")]
pub fn generate_png_with_colors(
    colors: [Color; 19],
    size_in_pixels: u16,
) -> Result<Vec<u8>, png::EncodingError> {
    let data = circles::calculate_png_data(size_in_pixels, colors);
    make_png_from_data(&data, size_in_pixels)
}

/// Data for small-sized identicon `png`, from `&[u8]` input slice,
/// larger image is generated first and then scaled down to fit the required size
///
/// Input slice could be of any length, as it gets hashed anyways;
/// typical input is a public key.
///
/// Function is expected to be used with image size values approximately 100 pix and below.
///
/// # Scaling factor
///
/// Scaling factor here is how much bigger the image gets generated, before it is scaled down.
///
/// ## Example
///
/// Let's generate a set of small identicons by scaling with different scaling factors.
///
/// Set the filter type to CatmullRom, and vary the scaling factor.
/// ```
/// use image::imageops::FilterType;
/// use plot_icon::generate_png_scaled_custom;
///
/// let alice: &[u8] = &[212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133, 88, 133, 76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125];
/// let size_in_pixels = 32;
/// let filter = FilterType::CatmullRom;
/// for scaling_factor in 1..=16 {
///     let content = generate_png_scaled_custom (alice, size_in_pixels, scaling_factor, filter).unwrap();
/// }
/// ```
/// For image size 32, the `scaling_factor = 1` results in strongly pixelated image,
/// it is identical to `generate_png` result.
/// With `scaling_factor = 2` image is already much less pixelated, off-centering still
/// visible; off-centering virtually disappears for `scaling factor = 4` and above,
/// after `scaling factor = 6` it is quite challenging to find any image differences at all.
///
/// # Filter type
///
/// [`FilterType`](https://docs.rs/image/latest/image/imageops/enum.FilterType.html) is the filter used during the scaling.
///
/// ## Example
///
/// Let's generate another set of small identicons by scaling with different filters.
///
/// Set the scaling factor to 8, and vary the filter.
/// ```
/// use image::imageops::FilterType;
/// use plot_icon::generate_png_scaled_custom;
///
/// let bob: &[u8] = &[142, 175, 4, 21, 22, 135, 115, 99, 38, 201, 254, 161, 126, 37, 252, 82, 135, 97, 54, 147, 201, 18, 144, 156, 178, 38, 170, 71, 148, 242, 106, 72];
/// let filter_set = [
///     FilterType::Nearest,
///     FilterType::Triangle,
///     FilterType::CatmullRom,
///     FilterType::Gaussian,
///     FilterType::Lanczos3,
/// ];
/// let scaling_factor = 8;
/// let size_in_pixels = 32;
/// for filter in filter_set.into_iter() {
///     let content = generate_png_scaled_custom (bob, size_in_pixels, scaling_factor, filter).unwrap();
/// }
/// ```
/// All identicons are readable,
/// the one made with `FilterType::Nearest` seems too pixelated,
/// the one made with `FilterType::Gaussian` seems a bit blurry.
///
/// # Selecting the defaults for Signer
///
/// To select the default values properly, a set of images with different scaling factors
/// was generated for each of the filters.
/// Signer at the moment uses 30 pix images.
///
/// ## Making image set to choose from
///
/// ```
/// use image::imageops::FilterType;
/// use plot_icon::generate_png_scaled_custom;
///
/// let alice: &[u8] = &[212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133, 88, 133, 76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125];
/// let bob: &[u8] = &[142, 175, 4, 21, 22, 135, 115, 99, 38, 201, 254, 161, 126, 37, 252, 82, 135, 97, 54, 147, 201, 18, 144, 156, 178, 38, 170, 71, 148, 242, 106, 72];
/// let filter_set = [
///     FilterType::Nearest,
///     FilterType::Triangle,
///     FilterType::CatmullRom,
///     FilterType::Gaussian,
///     FilterType::Lanczos3,
/// ];
/// let size_in_pixels = 30;
/// for i in 2..=10 {
///     for filter in filter_set.iter() {
///         for id_slice in [alice, bob].into_iter() {
///             let content = generate_png_scaled_custom (id_slice, size_in_pixels, i, *filter).unwrap();
///         }
///     }
/// }
/// ```
/// For now the default scaling factor is selected to be `5`,
/// the default filter is selected to be `FilterType::Lanczos3`.
///
#[cfg(feature = "pix")]
pub fn generate_png_scaled_custom(
    into_id: &[u8],
    size_in_pixels: u8,
    scaling_factor: u8,
    filter_type: FilterType,
) -> Result<Vec<u8>, IdenticonError> {
    let colors = colors::get_colors(into_id);
    generate_png_scaled_custom_with_colors(colors, size_in_pixels, scaling_factor, filter_type)
}

/// Data for small-sized identicon `png`, with given colors,
/// larger image is generated first and then scaled down to fit the required size
///
/// Makes regular identicons if properly generated color set is plugged in.
/// Also could be used to generate test identicon-like pictures.
///
/// Input [`Color`] set is in RGBA format.
///
/// Function is expected to be used with image size values approximately 100 pix and below.
///
/// ## Example
///
/// Make identicon with gray circles.
///
/// ```
/// use image::imageops::FilterType;
/// use plot_icon::{generate_png_scaled_custom_with_colors, colors::Color};
///
/// let rgb_value = 150;
/// let size_in_pixels = 30;
/// let scaling_factor = 5;
/// let filter_type = FilterType::Lanczos3;
/// let colors = [Color{red: rgb_value, green: rgb_value, blue: rgb_value, alpha: 255}; 19];
/// let content = generate_png_scaled_custom_with_colors(colors, size_in_pixels, scaling_factor, filter_type).unwrap();
/// ```
#[cfg(feature = "pix")]
pub fn generate_png_scaled_custom_with_colors(
    colors: [Color; 19],
    size_in_pixels: u8,
    scaling_factor: u8,
    filter_type: FilterType,
) -> Result<Vec<u8>, IdenticonError> {
    let data_large =
        generate_png_with_colors(colors, size_in_pixels as u16 * scaling_factor as u16)
            .map_err(IdenticonError::Png)?;
    let image_large = load_from_memory(&data_large).map_err(IdenticonError::Image)?;
    let image_small = resize(
        &image_large,
        size_in_pixels as u32,
        size_in_pixels as u32,
        filter_type,
    );
    make_png_from_data(&image_small, size_in_pixels as u16).map_err(IdenticonError::Png)
}

/// Data for small-sized identicon `png`, from `&[u8]` input slice,
/// with default settings used for Signer app
///
/// Generates larger image and then scales it down to fit the required size,
/// with default filter (`FilterType::Lanczos3`) and default scaling factor (`5`).
/// Function is unfallible. If `png` generation itself fails, is falls back to transparent 30x30 `png`.
///
/// ## Example
///
/// ```
/// use plot_icon::generate_png_scaled_default;
///
/// let alice: &[u8] = &[212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133, 88, 133, 76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125];
/// let filename = "test_pics/default_signer_alice.png";
/// let content = generate_png_scaled_default(alice);
/// assert!(std::fs::read(&filename).unwrap() == content, "Generated different image for {}!", filename);
/// ```
///
#[cfg(feature = "pix")]
pub fn generate_png_scaled_default(into_id: &[u8]) -> Vec<u8> {
    match generate_png_scaled_custom(into_id, SIZE_IN_PIXELS, SCALING_FACTOR, FILTER_TYPE) {
        Ok(a) => a,
        Err(_) => EMPTY_PNG.to_vec(),
    }
}

/// Helper function to write calculated pixel-by-pixel `png` pixel data in `png` format, header and all
#[cfg(feature = "pix")]
fn make_png_from_data(data: &[u8], size_in_pixels: u16) -> Result<Vec<u8>, png::EncodingError> {
    let mut out: Vec<u8> = Vec::new();
    let mut encoder = png::Encoder::new(&mut out, size_in_pixels as u32, size_in_pixels as u32);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;
    writer.write_image_data(data)?;
    drop(writer);
    Ok(out)
}

/// Errors in `png` identicon generation
#[derive(Debug)]
#[cfg(feature = "pix")]
pub enum IdenticonError {
    /// [`png::EncodingError`](https://docs.rs/png/latest/png/enum.EncodingError.html)
    ///
    /// From `png` crate, could appear on writing the pixel data into `png`,
    /// generally should not happen, since the `png` parameters are matching the pixel data generated
    Png(png::EncodingError),
    /// [`image::ImageError`](https://docs.rs/image/latest/image/error/enum.ImageError.html)
    ///
    /// From `image` crate, could appear on loading freshly made `png` into `DynamicImage`,
    /// very unlikely
    Image(image::ImageError),
}

#[cfg(feature = "pix")]
impl IdenticonError {
    /// displaying error text
    pub fn show(&self) -> String {
        match &self {
            IdenticonError::Png(e) => format!("Error encoding data into png format: {}", e),
            IdenticonError::Image(e) => format!(
                "Error transforming png data into DynamicImage format: {}",
                e
            ),
        }
    }
}

#[cfg(feature = "pix")]
impl std::fmt::Display for IdenticonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.show())
    }
}

/// Identicon [`svg::Document`](https://docs.rs/svg/latest/svg/type.Document.html)
/// data, from `&[u8]` input slice
///
/// Input slice could be of any length, as it gets hashed anyways;
/// typical input is a public key.
///
/// ## Example
///
/// ```
/// use plot_icon::generate_svg;
///
/// let alice: &[u8] = &[212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133, 88, 133, 76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125];
/// let svg_document = generate_svg (&alice);
/// let mut svg_expected_content = String::new();
/// svg::open("test_pics/alice.svg", &mut svg_expected_content).unwrap();
/// assert!(svg_document.to_string() == svg_expected_content, "Expected different svg content, got: {}", svg_document);
/// ```
#[cfg(feature = "vec")]
pub fn generate_svg(into_id: &[u8]) -> Document {
    let colors = colors::get_colors(into_id);
    generate_svg_with_colors(colors)
}

/// Identicon [`svg::Document`](https://docs.rs/svg/latest/svg/type.Document.html)
/// data, with given colors
///
/// Makes regular identicons if properly generated color set is plugged in.
/// Also could be used to generate test identicon-like pictures.
///
/// Input [`Color`] set is in RGBA format.
///
/// ## Example
///
/// ```
/// use plot_icon::{generate_svg_with_colors, colors::Color};
///
/// let colors = [Color{red: 200, green: 200, blue: 200, alpha: 255}; 19];
/// let svg_document = generate_svg_with_colors(colors);
/// ```
#[cfg(feature = "vec")]
pub fn generate_svg_with_colors(colors: [Color; 19]) -> Document {
    let unit = 10; // svg is vector format, the unit size is arbitrary, and does not influence the outcome
    let mut document = Document::new().set("viewBox", (-unit, -unit, 2 * unit, 2 * unit));
    let data = circles::calculate_svg_data(unit as f32, colors);
    for x in data.into_iter() {
        document = document.add(x);
    }
    document
}
