use plot_icon::{plot_png_from_hex, plot_png_from_base58, plot_svg_from_hex, plot_svg_from_base58, png_data_scaled, png_data_from_vec};

const HALF_PNG: i32 = 500;
const HALF_SVG: i32 = 32;
const HEX_WESTEND: &str = "e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
const BASE_ALICE: &str = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";

fn main() {
    let filename = "westeng_genesis_hash.png";
    match plot_png_from_hex (HEX_WESTEND, HALF_PNG, filename) {
        Ok(()) => println!("Done!"),
        Err(e) => println!("Error. {}", e),
    }
    let filename = "westeng_genesis_hash.svg";
    match plot_svg_from_hex (HEX_WESTEND, HALF_SVG, filename) {
        Ok(()) => println!("Done!"),
        Err(e) => println!("Error. {}", e),
    }
    let filename = "alice.png";
    match plot_png_from_base58 (BASE_ALICE, HALF_PNG, filename) {
        Ok(()) => println!("Done!"),
        Err(e) => println!("Error. {}", e),
    }
    let filename = "alice.svg";
    match plot_svg_from_base58 (BASE_ALICE, HALF_SVG, filename) {
        Ok(()) => println!("Done!"),
        Err(e) => println!("Error. {}", e),
    }
    let filename = "westend_32pix_as_is.png";
    let vec = vec![225, 67, 242, 56, 3, 172, 80, 232, 246, 248, 230, 38, 149, 209, 206, 158, 78, 29, 104, 170, 54, 193, 205, 44, 253, 21, 52, 2, 19, 243, 66, 62];
    match png_data_from_vec (&vec, 20) {
        Ok(content) => std::fs::write(filename, &content).unwrap(),
        Err(e) => println!("Error. {}", e),
    }
    let filename = "westend_32pix_scaled.png";
    match png_data_scaled (&vec, 20) {
        Ok(content) => std::fs::write(filename, &content).unwrap(),
        Err(e) => println!("Error. {}", e),
    }
}




