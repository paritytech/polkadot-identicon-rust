use plot_icon::{plot_png_from_hex, plot_png_from_base58, plot_svg_from_hex, plot_svg_from_base58};

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
}




