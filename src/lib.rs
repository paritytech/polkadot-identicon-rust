//! Generates 19-circle identicons in png and in svg format from `&[u8]` input slice. Slice length could be arbitrary, as identicon generation starts with input hashing. Typical input is a public key.  
//!
//! The identicon color scheme and elements arrangement follow the published javascript [code](https://github.com/paritytech/oo7/blob/master/packages/polkadot-identicon/src/index.jsx) for polkadot identicon generation. This crate is intended mainly for use by [Signer](https://github.com/paritytech/parity-signer).  
//!
//! Feature `"pix"` supports generation of png images, feature `"vec"` - generation of svg images. Both are made available by default.  

#[cfg(feature = "pix")]
use image::{
    imageops::{resize, FilterType},
    load_from_memory,
};

#[cfg(feature = "vec")]
use svg::Document;

pub mod circles;
pub mod colors;

#[cfg(feature = "pix")]
const SCALING_FACTOR: u8 = 8;
#[cfg(feature = "pix")]
const FILTER_TYPE: FilterType = FilterType::CatmullRom;

/// Generate polkadot identicon png data in u8 vector format, from `&[u8]` input slice
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
/// let size_set: Vec<u16> = vec![16, 32, 45, 64, 128, 256, 512];
/// for size_in_pixels in size_set.into_iter() {
///     let filename = format!("test_pics/size_set_alice_{}pix.png", size_in_pixels);
///     let content = generate_png(alice, size_in_pixels).unwrap();
///     std::fs::write(&filename, &content).unwrap();
///     let image = load_from_memory(&content).unwrap();
///     assert!(image.width() == size_in_pixels as u32);
///     assert!(image.height() == size_in_pixels as u32);
/// }
/// ```
#[cfg(feature = "pix")]
pub fn generate_png(into_id: &[u8], size_in_pixels: u16) -> Result<Vec<u8>, png::EncodingError> {
    let colors = colors::get_colors(into_id);
    let data = circles::calculate_png_data(size_in_pixels, colors);
    make_png_from_data(&data, size_in_pixels)
}

/// Generate data for small-sized identicon png, from `&[u8]` input slice,
/// by first making a larger image and then scaling it down to fit the required size
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
/// Set the filter type to default CatmullRom, and vary the scaling factor.
/// ```
/// use image::imageops::FilterType;
/// use plot_icon::generate_png_scaled_custom;
///
/// let alice: &[u8] = &[212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133, 88, 133, 76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125];
/// let size_in_pixels = 32;
/// let filter = FilterType::CatmullRom;
/// for scaling_factor in 1..=16 {
///     let filename = format!("test_pics/scaled_{}x_alice_32pix_catmullrom.png", scaling_factor);
///     let content = generate_png_scaled_custom (alice, size_in_pixels, scaling_factor, filter).unwrap();
///     std::fs::write(&filename, &content).unwrap();
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
/// Set the scaling factor to default 8, and vary the filter.
/// ```
/// use image::imageops::FilterType;
/// use plot_icon::generate_png_scaled_custom;
///
/// let bob: &[u8] = &[142, 175, 4, 21, 22, 135, 115, 99, 38, 201, 254, 161, 126, 37, 252, 82, 135, 97, 54, 147, 201, 18, 144, 156, 178, 38, 170, 71, 148, 242, 106, 72];
/// let filter_set: Vec<(FilterType, &str)> = vec![
///     (FilterType::Nearest, "nearest"),
///     (FilterType::Triangle, "triangle"),
///     (FilterType::CatmullRom, "catmullrom"),
///     (FilterType::Gaussian, "gaussian"),
///     (FilterType::Lanczos3, "lanczos3")
/// ];
/// let scaling_factor = 8;
/// let size_in_pixels = 32;
/// for filter_bundle in filter_set.into_iter() {
///     let filename = format!("test_pics/file_set_bob_32pix_scaled_8x_{}.png", filter_bundle.1);
///     let content = generate_png_scaled_custom (bob, size_in_pixels, scaling_factor, filter_bundle.0).unwrap();
///     std::fs::write(&filename, &content).unwrap();
/// }
/// ```
/// All identicons are readable,
/// the one made with `FilterType::Nearest` seems too pixelated,
/// the one made with `FilterType::Gaussian` seems a bit blurry.
///
#[cfg(feature = "pix")]
pub fn generate_png_scaled_custom(
    into_id: &[u8],
    size_in_pixels: u8,
    scaling_factor: u8,
    filter_type: FilterType,
) -> Result<Vec<u8>, IdenticonError> {
    let data_large = generate_png(into_id, size_in_pixels as u16 * scaling_factor as u16)
        .map_err(IdenticonError::Png)?;
    let image_large = load_from_memory(&data_large).map_err(IdenticonError::Image)?;
    let image_small = resize(
        &image_large,
        size_in_pixels as u32,
        size_in_pixels as u32,
        filter_type,
    );
    make_png_from_data(&image_small.to_vec(), size_in_pixels as u16).map_err(IdenticonError::Png)
}

/// Generate data for small-sized identicon png, from `&[u8]` input slice,
/// with default settings used for Signer app
///
/// Generates larger image and then scales it down to fit the required size,
/// with default filter (cubic) and default scaling factor (8).
///
/// ## Example
///
/// ```
/// use plot_icon::generate_png_scaled_default;
///
/// let alice: &[u8] = &[212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133, 88, 133, 76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125];
/// let size_in_pixels = 32;
/// let filename = "test_pics/default_signer_alice_32pix.png";
/// let content = generate_png_scaled_default(alice, size_in_pixels).unwrap();
/// std::fs::write(&filename, &content).unwrap();
/// ```
///
#[cfg(feature = "pix")]
pub fn generate_png_scaled_default(
    into_id: &[u8],
    size_in_pixels: u8,
) -> Result<Vec<u8>, IdenticonError> {
    generate_png_scaled_custom(into_id, size_in_pixels, SCALING_FACTOR, FILTER_TYPE)
}

/// Helper function to write calculated pixel-by-pixel png data in png format, header and all
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

/// Errors in png identicon generation
#[derive(Debug)]
#[cfg(feature = "pix")]
pub enum IdenticonError {
    /// [`png::EncodingError`](https://docs.rs/png/latest/png/enum.EncodingError.html)
    ///
    /// From `png` crate, could appear on writing the pixel data into png,
    /// generally should not happen, since the png parameters are matching the pixel data generated
    Png(png::EncodingError),
    /// [`image::ImageError`](https://docs.rs/image/latest/image/error/enum.ImageError.html)
    ///
    /// From `image` crate, could appear on loading freshly made png into `DynamicImage`,
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

/// Generate identicon [`svg::Document`](https://docs.rs/svg/latest/svg/type.Document.html)
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
/// let filename = "test_pics/alice.svg";
/// svg::save(filename, &svg_document).unwrap();
/// ```
#[cfg(feature = "vec")]
pub fn generate_svg(into_id: &[u8]) -> Document {
    let unit = 100; // svg is vector format, the unit size is arbitrary, and does not influence the outcome
    let mut document = Document::new().set("viewBox", (-unit, -unit, 2 * unit, 2 * unit));
    let colors = colors::get_colors(into_id);
    let data = circles::calculate_svg_data(unit as f32, colors);
    for x in data.into_iter() {
        document = document.add(x);
    }
    document
}

#[cfg(test)]
#[cfg(feature = "pix")]
mod tests {
    use super::*;

    const ALICE: &[u8] = &[
        212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133, 88,
        133, 76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125,
    ];

    #[test]
    #[ignore]
    fn no_overflow_in_png() {
        let content_maybe = generate_png(ALICE, u16::MAX);
        assert!(
            content_maybe.is_ok(),
            "Failed to generate largest allowed png."
        );
    }

    #[test]
    #[ignore]
    fn no_overflow_in_scaled_png() {
        let content_maybe = generate_png_scaled_custom(ALICE, u8::MAX, u8::MAX, FILTER_TYPE);
        assert!(
            content_maybe.is_ok(),
            "Failed to generate largest allowed scaled png."
        );
    }
}
