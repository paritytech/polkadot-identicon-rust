use png;
use svg::Document;
use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use base58::FromBase58;
use anyhow::anyhow;

mod colors;
use colors::get_colors_from_vec;

mod circles;
use circles::{calculate_png_data, calculate_svg_data};


/// plot png icon from id as u8 vector
pub fn plot_png_from_vec (into_id: &Vec<u8>, halfsize_in_pixels: i32, filename: &str) -> anyhow::Result<()> {
    let path = Path::new(filename);
    let file = match File::create(path) {
        Ok(x) => x,
        Err(e) => return Err(anyhow!("Error writing file at {}. {}", filename, e)),
    };
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, (halfsize_in_pixels*2+1) as u32, (halfsize_in_pixels*2+1) as u32);
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = match encoder.write_header() {
        Ok(x) => x,
        Err(e) => return Err(anyhow!("Error writing header. {}", e)),
    };
    
    let colors = get_colors_from_vec(&into_id);
    let data = calculate_png_data (halfsize_in_pixels, colors);
    
    match writer.write_image_data(&data) {
        Ok(()) => Ok(()),
        Err(e) => return Err(anyhow!("Error writing image data. {}", e)),
    }
}


/// plot svg icon from id as u8 vector
pub fn plot_svg_from_vec (into_id: &Vec<u8>, halfsize: i32, filename: &str) -> anyhow::Result<()> {
    let mut document = Document::new()
        .set("viewBox", (-halfsize, -halfsize, 2*halfsize, 2*halfsize));
    let colors = get_colors_from_vec(&into_id);
    let data = calculate_svg_data (halfsize, colors);
    for x in data.into_iter() {
        document = document.add(x);
    }
    match svg::save(filename, &document) {
        Ok(()) => Ok(()),
        Err(e) => return Err(anyhow!("Error writing file. {}", e)),
    }
}


/// plot png icon from hex line
pub fn plot_png_from_hex (hex_line: &str, halfsize_in_pixels: i32, filename: &str) -> anyhow::Result<()> {
    let into_id = unhex(hex_line)?;
    plot_png_from_vec (&into_id, halfsize_in_pixels, filename)
}

/// plot svg icon from hex line
pub fn plot_svg_from_hex (hex_line: &str, halfsize: i32, filename: &str) -> anyhow::Result<()> {
    let into_id = unhex(hex_line)?;
    plot_svg_from_vec (&into_id, halfsize, filename)
}

/// plot png icon from base58 line
pub fn plot_png_from_base58 (base58_line: &str, halfsize_in_pixels: i32, filename: &str) -> anyhow::Result<()> {
    let into_id = unbase(base58_line)?;
    plot_png_from_vec (&into_id, halfsize_in_pixels, filename)
}

/// plot svg icon from base58 line
pub fn plot_svg_from_base58 (base58_line: &str, halfsize: i32, filename: &str) -> anyhow::Result<()> {
    let into_id = unbase(base58_line)?;
    plot_svg_from_vec (&into_id, halfsize, filename)
}

/// helper function to unhex input
fn unhex (hex_line: &str) -> anyhow::Result<Vec<u8>> {
    let hex_line = {
        if hex_line.starts_with("0x") {&hex_line[2..]}
        else {hex_line}
    };
    match hex::decode(&hex_line) {
        Ok(x) => Ok(x),
        Err(_) => return Err(anyhow!("String is not in hex format")),
    }
}


/// helper function to unbase input
fn unbase (base58_line: &str) -> anyhow::Result<Vec<u8>> {
    let prep = match base58_line.from_base58() {
        Ok(x) => x,
        Err(_) => return Err(anyhow!("String is not in base58 format")),
    };
    Ok(prep[1..prep.len()-2].to_vec()) // cut off base58 prefix and last two units that are part of the hash
}
