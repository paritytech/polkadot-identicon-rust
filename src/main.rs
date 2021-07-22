use plot_icon::{plot_icon_from_hex, plot_icon_from_base58};

fn main() {

    let halfsize_in_pixels: i32 = 500;
    let filename = "westend_genhash_as_icon";
    let hex_line = "e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";

    match plot_icon_from_hex (hex_line, halfsize_in_pixels, filename) {
        Ok(()) => println!("Done!"),
        Err(e) => println!("Error. {}", e),
    }
    
    let filename = "test_alice_from_base58";
    let base58_line = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";
    
    match plot_icon_from_base58 (base58_line, halfsize_in_pixels, filename) {
        Ok(()) => println!("Done!"),
        Err(e) => println!("Error. {}", e),
    }

}




