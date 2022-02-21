use base58::FromBase58;
use hex;
use image::imageops::FilterType;
use svg;

use plot_icon::{generate_png, generate_png_scaled_custom, generate_png_scaled_default, generate_svg};

const ALICE: &[u8] = &[212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133, 88, 133, 76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125];
const BOB: &[u8] = &[142, 175, 4, 21, 22, 135, 115, 99, 38, 201, 254, 161, 126, 37, 252, 82, 135, 97, 54, 147, 201, 18, 144, 156, 178, 38, 170, 71, 148, 242, 106, 72];
const ALICE_HEX: &str = "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";
const ALICE_BASE58: &str = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";


/// # Generating an identicon without scaling
/// Start with base58 string for public key,
/// generate and print identicon with varying halfsize.
/// Identicon with sufficient halfsize looks acceptable without rescaling.
fn plot_png_identicon_alice() {
    let unbased = ALICE_BASE58.from_base58().unwrap();
    let alice_data = &unbased[1..unbased.len()-2];
    let halfsize_set: Vec<u16> = vec![8, 16, 32, 64, 128, 256];
    for halfsize_in_pixels in halfsize_set.into_iter() {
        let filename = format!("examples/size_set_alice_{}pix.png", 2*halfsize_in_pixels);
        let content = generate_png(alice_data, halfsize_in_pixels).unwrap();
        std::fs::write(&filename, &content).unwrap();
    }
}

/// # Generating small identicon (with default scaling)
/// This function is the one to be used in Signer.
/// Start with &[u8] public key,
/// generate and print identicon with halfsize 16.
/// Identicon looks acceptable.
fn plot_small_rescaled_png_identicon_alice() {
    let halfsize_in_pixels = 16;
    let filename = "examples/default_signer_alice_32pix.png";
    let content = generate_png_scaled_default(ALICE, halfsize_in_pixels).unwrap();
    std::fs::write(&filename, &content).unwrap();
}

/// # Generating small identicons by scaling with different scaling factors
/// Set the filter type to default CatmullRom, and vary the scaling factor.
/// For halfsize 16, the scaling_factor=1 will result in strongly pixelated image,
/// scaling_factor=2 will be already acceptable, and
/// for higher scaling factors it is quite challenging to find any difference at all.
fn scaling_factor_test() {
    let filter = FilterType::CatmullRom;
    let scaling_set: Vec<u8> = vec![1, 2, 4, 8, 16];
    let halfsize_in_pixels = 16;
    for scaling_factor in scaling_set.into_iter() {
        let filename = format!("examples/scaled_{}x_alice_32pix_catmullrom.png", scaling_factor);
        let content = generate_png_scaled_custom (ALICE, halfsize_in_pixels, scaling_factor, filter).unwrap();
        std::fs::write(&filename, &content).unwrap();
    }
}

/// # Generating small identicons by scaling with different filters
/// Set the scaling factor to default 8, and vary the filter.
/// All identicons are readable,
/// the one made with FilterType::Nearest seems too pixelated,
/// the one made with FilterType::Gaussian seems a bit blurry.
fn filter_test() {
    let filter_set: Vec<(FilterType, &str)> = vec![(FilterType::Nearest, "nearest"),(FilterType::Triangle, "triangle"),(FilterType::CatmullRom, "catmullrom"),(FilterType::Gaussian, "gaussian"),(FilterType::Lanczos3, "lanczos3")];
    let scaling_factor = 8;
    let halfsize_in_pixels = 16;
    for filter_bundle in filter_set.into_iter() {
        let filename = format!("examples/file_set_bob_32pix_scaled_8x_{}.png", filter_bundle.1);
        let content = generate_png_scaled_custom (BOB, halfsize_in_pixels, scaling_factor, filter_bundle.0).unwrap();
        std::fs::write(&filename, &content).unwrap();
    }
}

/// # Generating and printing identicon in svg format
/// Start with hexadecimal string for public key.
/// Function `generate_svg` produces svg::Document,
/// that could be saved with svg::save.
fn plot_svg_identicon_alice() {
    let alice_data = hex::decode(ALICE_HEX).unwrap();
    let svg_document = generate_svg (&alice_data);
    let filename = "examples/alice.svg";
    svg::save(filename, &svg_document).unwrap();
}

fn main() {
    plot_png_identicon_alice(); // Making large png file (without rescaling)
    plot_small_rescaled_png_identicon_alice(); // Making small png file (with default scaling)
    scaling_factor_test(); // Testing different scaling factors
    filter_test(); // Testing different filter types
    plot_svg_identicon_alice(); // Generating svg identicon for Alice, from hexadecimal string
}


