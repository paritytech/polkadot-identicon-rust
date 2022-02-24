use blake2_rfc::blake2b::blake2b;
use palette::{FromColor, FromComponent, Hsl, RgbHue, Srgb};

use crate::circles::FOREGROUND_COLOR;

/// Struct to store default coloring schemes
struct SchemeElement {
    freq: u8,
    colors: [usize; 19],
}

/// Function to set default coloring schemes, taken as is from js code
#[rustfmt::skip]
fn default_schemes() -> Vec<SchemeElement> {
    vec![
        SchemeElement {
            // "target"
            freq: 1,
            colors: [0, 28, 0, 0, 28, 0, 0, 28, 0, 0, 28, 0, 0, 28, 0, 0, 28, 0, 1],
        },
        SchemeElement {
            // "cube",
            freq: 20,
            colors: [0, 1, 3, 2, 4, 3, 0, 1, 3, 2, 4, 3, 0, 1, 3, 2, 4, 3, 5],
        },
        SchemeElement {
            // "quazar",
            freq: 16,
            colors: [1, 2, 3, 1, 2, 4, 5, 5, 4, 1, 2, 3, 1, 2, 4, 5, 5, 4, 0],
        },
        SchemeElement {
            // "flower",
            freq: 32,
            colors: [0, 1, 2, 0, 1, 2, 0, 1, 2, 0, 1, 2, 0, 1, 2, 0, 1, 2, 3],
        },
        SchemeElement {
            // "cyclic",
            freq: 32,
            colors: [0, 1, 2, 3, 4, 5, 0, 1, 2, 3, 4, 5, 0, 1, 2, 3, 4, 5, 6],
        },
        SchemeElement {
            // "vmirror",
            freq: 128,
            colors: [0, 1, 2, 3, 4, 5, 3, 4, 2, 0, 1, 6, 7, 8, 9, 7, 8, 6, 10],
        },
        SchemeElement {
            // "hmirror",
            freq: 128,
            colors: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 8, 6, 7, 5, 3, 4, 2, 11],
        },
    ]
}

/// Function to calculate identicon colors from `&[u8]` input slice.
/// Total 19 colors are always produced.
pub fn get_colors(into_id: &[u8]) -> [[u8; 4]; 19] {
    let into_zero = &[0u8; 32];
    let zero = blake2b(64, &[], into_zero).as_bytes().to_vec();

    let id_prep = blake2b(64, &[], into_id).as_bytes().to_vec();

    let mut id: Vec<u8> = Vec::new();
    for (i, x) in id_prep.iter().enumerate() {
        let new = x.wrapping_sub(zero[i]);
        id.push(new);
    }

    // Since `id[29]` is u8, `sat` could range from 30 to 109, i.e. it always fits into u8.
    // Transformation of id[29] into u16 is to avoid overflow in multiplication (wrapping could be used, but is more bulky).
    // TODO For color calculation `sat` is used as saturation in percents
    // (this is taken as is from js code).
    // However, this way `sat_component` could have values above 1.00.
    // Palette crate does not check at this moment that `sat_component` is not overflowing 1.00, and produces
    // some kind of resulting color.
    // Need to find out what should have happened if the sat values are above 100.
    let sat = (((id[29] as u16 * 70 / 256 + 26) % 80) + 30) as u8;
    let sat_component: f64 = (sat as f64) / 100.0;

    // calculating palette: set of 32 RGBA colors to be used in drawing
    // only id vector is used for this calculation
    let mut my_palette: Vec<[u8; 4]> = Vec::new();
    for (i, x) in id.iter().enumerate() {
        let b = x.wrapping_add((i as u8 % 28).wrapping_mul(58));
        let new = match b {
            0 => [4, 4, 4, 255],
            255 => FOREGROUND_COLOR, // transparent
            _ => {
                // HSL color hue in degrees
                // calculated as integer, same as in js code
                // transformation to u16 is done to avoid overflow
                let h = (b as u16 % 64 * 360) / 64;
                // recalculated into RgbHue, to be used as HSL hue component
                let h_component = RgbHue::from_degrees(h as f64);

                // HSL lightness in percents
                let l: u8 = match b / 64 {
                    0 => 53,
                    1 => 15,
                    2 => 35,
                    _ => 75,
                };
                // recalculated in HSL lightness component (component range is 0.00 to 1.00)
                let l_component: f64 = (l as f64) / 100.0;

                // defining HSL color
                let color_hsl = Hsl::new(h_component, sat_component, l_component);

                // transforming HSL color into RGB color, possibly lossy, TODO check if too lossy
                let color_srgb = Srgb::from_color(color_hsl);

                // getting red, green, blue components, transforming them in 0..255 range of u8
                let red = u8::from_component(color_srgb.red);
                let green = u8::from_component(color_srgb.green);
                let blue = u8::from_component(color_srgb.blue);

                // finalize color to add to palette, not transparent
                [red, green, blue, 255]
            }
        };
        my_palette.push(new);
    }

    // loading default coloring schemes
    let schemes = default_schemes();

    // `total` is the sum of frequencies for all scheme elements in coloring schemes,
    // in current setting is always 357
    let mut total = 0;
    for x in schemes.iter() {
        total += x.freq as u32;
    }

    // `d` is used to determine the coloring scheme to be used.
    // Transformation into u32 is used to avoid overflow.
    let d = (id[30] as u32 + (id[31] as u32) * 256) % total;

    // determining the coloring scheme to be used
    let my_scheme = choose_scheme(schemes, d);

    // calculating rotation for the coloring scheme
    let rot = (id[28] % 6) * 3;

    // picking colors from palette using coloring scheme with rotation applied
    let mut my_colors: Vec<[u8; 4]> = Vec::with_capacity(19);
    for i in 0..19 {
        let num_color = {
            if i < 18 {
                (i + rot) % 18
            } else {
                18
            }
        } as usize;
        let num_palette = my_scheme.colors[num_color];
        let color = my_palette[num_palette];
        my_colors.push(color);
    }

    my_colors.try_into().expect("always generate 19-element set")
}

/// Function to choose the coloring scheme based on value d.
/// Note that d is calculated as remainder of division by total sum of frequencies,
/// so it can not exceed the total sum of frequencies
fn choose_scheme(schemes: Vec<SchemeElement>, d: u32) -> SchemeElement {
    let mut sum = 0;
    let mut found_scheme = None;
    for x in schemes.into_iter() {
        sum += x.freq as u32;
        if d < sum {
            found_scheme = Some(x);
            break;
        }
    }
    found_scheme.expect("should always be determined: d is calculated as remainder of division by total sum of frequencies, so it can not exceed the total sum of frequencies")
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALICE: &[u8] = &[
        212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133, 88,
        133, 76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125,
    ]; // Alice public key; corresponds to hexadecimal "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d" and base58 "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY" in westend network
    const BOB: &[u8] = &[
        142, 175, 4, 21, 22, 135, 115, 99, 38, 201, 254, 161, 126, 37, 252, 82, 135, 97, 54, 147,
        201, 18, 144, 156, 178, 38, 170, 71, 148, 242, 106, 72,
    ]; // Bob public key; corresponds to hexadecimal "8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48" and base58 "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty" in westend network

    /// made with a color picking GIMP tool using the icon from polkadot website
    fn alice_website() -> Vec<[u8; 4]> {
        vec![
            [165, 227, 156, 255],
            [60, 40, 17, 255],
            [184, 68, 202, 255],
            [139, 39, 88, 255],
            [135, 68, 202, 255],
            [225, 156, 227, 255],
            [139, 39, 88, 255],
            [135, 68, 202, 255],
            [184, 68, 202, 255],
            [165, 227, 156, 255],
            [60, 40, 17, 255],
            [162, 202, 68, 255],
            [39, 139, 139, 255],
            [187, 202, 68, 255],
            [38, 60, 17, 255],
            [39, 139, 139, 255],
            [187, 202, 68, 255],
            [162, 202, 68, 255],
            [61, 39, 139, 255],
        ]
    }

    /// made with a color picking GIMP tool using the icon from polkadot website
    fn bob_website() -> Vec<[u8; 4]> {
        vec![
            [58, 120, 61, 255],
            [200, 214, 169, 255],
            [214, 169, 182, 255],
            [36, 52, 25, 255],
            [127, 93, 177, 255],
            [214, 169, 182, 255],
            [58, 120, 61, 255],
            [200, 214, 169, 255],
            [52, 25, 30, 255],
            [113, 177, 93, 255],
            [58, 120, 114, 255],
            [58, 120, 108, 255],
            [118, 93, 177, 255],
            [25, 52, 39, 255],
            [58, 120, 108, 255],
            [113, 177, 93, 255],
            [58, 120, 114, 255],
            [52, 25, 30, 255],
            [33, 25, 52, 255],
        ]
    }

    #[test]
    fn colors_alice() {
        let alice_calculated = get_colors(ALICE);
        assert!(
            alice_website() == alice_calculated,
            "Got different Alice colors:\n{:?}",
            alice_calculated
        );
    }

    #[test]
    fn colors_bob() {
        let bob_calculated = get_colors(BOB);
        assert!(
            bob_website() == bob_calculated,
            "Got different Bob colors:\n{:?}",
            bob_calculated
        );
    }
}
