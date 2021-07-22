use png;
use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use base58::FromBase58;

mod colors;
use colors::get_colors_from_vec;

mod circles;
use circles::calculate_picture_data;


/// plot png icon from id as u8 vector
pub fn plot_icon_from_vec (into_id: &Vec<u8>, halfsize_in_pixels: i32, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(filename);
    let file = File::create(path)?;
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, (halfsize_in_pixels*2+1) as u32, (halfsize_in_pixels*2+1) as u32);
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;
    
    let colors = get_colors_from_vec(&into_id);
    let data = calculate_picture_data (halfsize_in_pixels, colors);
    
    writer.write_image_data(&data)?;
    
    Ok(())
}


/// plot png icon from hex line
pub fn plot_icon_from_hex (hex_line: &str, halfsize_in_pixels: i32, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let hex_line = {
        if hex_line.starts_with("0x") {&hex_line[2..]}
        else {hex_line}
    };
    let into_id = match hex::decode(&hex_line) {
        Ok(x) => x,
        Err(_) => return Err(Box::from("String is not in hex format")),
    };
    plot_icon_from_vec (&into_id, halfsize_in_pixels, filename)
}


/// plot png icon from base58 line
pub fn plot_icon_from_base58 (base58_line: &str, halfsize_in_pixels: i32, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let into_id_prep = match base58_line.from_base58() {
        Ok(x) => x,
        Err(_) => return Err(Box::from("String is not in base58 format")),
    };
    let into_id = into_id_prep[1..into_id_prep.len()-2].to_vec(); // cut off base58 prefix and last two units that are part of the hash
    plot_icon_from_vec (&into_id, halfsize_in_pixels, filename)
}

