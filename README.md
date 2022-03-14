
# Crate `plot_icon`

## Overview

This is a lib crate for generating standard 19-circle icons in `png` and in `svg` format.  
![identicon](./src/identicon_example.svg)

Output is `Vec<u8>` `png` data, or `svg::Document` with `svg` data, both could be easily printed into files.  

The identicon color scheme and elements arrangement follow the published javascript [code](https://github.com/paritytech/oo7/blob/master/packages/polkadot-identicon/src/index.jsx) for polkadot identicon generation. This crate is intended mainly for use by [Signer](https://github.com/paritytech/parity-signer).  


## Input

Identicon is generated for `&[u8]` input slice. During identicon generation, this input slice gets hashed, therefore, any length would be acceptable.  

Typical input slice is a public key. Public key is often encountered as a hexadecimal string (`d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d`) or as a base58 network-specific string (`5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY`), both could be easily transformed into `&[u8]` input.  

Crate also supports generation of identicon-like images with user-provided colors in RGBA format.  


## PNG

Signer uses images in `png` format, since `svg` format is not sufficiently supported on devices side and might be non-deterministic. Therefore, this crate sticks mostly to `png` generation. Feature `"pix"` (enabled by default) enables generation of `png` images.  

Function `generate_png` produces `png` data for identicon, and requires:  
- `&[u8]` slice  
- target image size in pixels (`u16`)  
`png` images are generated pixel-by-pixel, and the quality of final image is determined by the image size. Each `png` pixel (with integer coordinates) falling within the identicon circle element (with float circle parameters) gets the color of the circle. Below certain image size (approximately 100 pix) the circles become too pixelated. Also, images with even number of pixels size are off-centered by a pixel.  

Signer needs small `png` identicon icons. Exact parameters are yet TBD (at the moment, identicons are 30 pix and device-independent), however, the straightforward approach with `generate_png` does not produce acceptable results.  

Possible solution is to generate larger identicon and then scale it down in Signer frontend, but it was noticed that the scaling results (pixelation, color distribution) are device-dependent and although a minor thing, it should definitely be avoided in *identicon*.  

To generate reproducible small identicons, the rescaling is performed within the crate. A larger `png` is generated, and then scaled down to originally desired size. This procedure results in both less pixelated circles and compensated off-centering.  

Function `generate_png_scaled_custom` performs the scaling with custom parameters, and requires:  
- `&[u8]` slice  
- **target** identicon size in pixels (`u8` - it is for small identicons, after all)  
- scaling factor (`u8`), how much larger the larger `png` actually is  
- filter ([`FilterType`](https://docs.rs/image/latest/image/imageops/enum.FilterType.html)) used for image resize  

The scaling factor reasonable values are in range `[4..=8]`, below it the pixelation persists, above it the images are not visibly improving anymore, and may even seem blurry.  

All filters produce reasonable results, except `FilterType::Nearest` that yields visibly distorted images and therefore is not recommended.  

Function `generate_png_scaled_default` performs the scaling with default scaling parameters (scaling factor `5` and filter `FilterType::Lanczos3`) for image with default Signer identicon size (30 pix), and requires only:  
- `&[u8]` slice  
If somehow the generation of the identicon fails, function outputs default-sized (30x30) transparent `png` image, i.e. it never produces an error.  

Function `generate_png_with_colors` is similar to `generate_png`, but accepts identicon colors directly, and does not generate color set itself. This is intended mainly for tests. Function `generate_png_with_colors` requires:  
- `[[u8; 4]; 19]` 19-element set of colors in RGBA format  
- target image size in pixels (`u16`)  

Function `generate_png_scaled_custom_with_colors` is similar to `generate_png_scaled_custom`, but accepts identicon colors directly, and does not generate color set itself. This is intended mainly for tests. Function `generate_png_scaled_custom_with_colors` requires:  
- `[[u8; 4]; 19]` 19-element set of colors in RGBA format  
- target identicon size in pixels (`u8`)  
- scaling factor (`u8`)  
- filter ([`FilterType`](https://docs.rs/image/latest/image/imageops/enum.FilterType.html)) used for image resize  


## SVG

Feature `"vec"` (enabled by default) enables infallible generation of identicon pictures in `svg` format. Since `svg` is a vector format, no image size parameters are needed.

Function `generate_svg` reqires only `&[u8]` input slice.  

Function `generate_svg_with_colors` uses pre-set colors and is intended mainly for tests. It requires only the color set (`[[u8; 4]; 19]` 19-element set of colors in RGBA format).  


## Tests and Examples

Tests in `colors.rs` module check if the color sets calculated for Alice and Bob are identical to the colors in the corresponding well-known icons.  

Doc tests in `lib.rs` produce various test pics, both png (through different functions and parameters) and `svg`.  


## Notes

There are several uncertainties about how the original published code was designed to work, those should be clarified, eventually.  

For example, calculated HSL color saturation could range 30..109, and is processed as percents. Crate `palette` (currently used here) processes saturation values over 100 as percents over 100, and gives some results (slightly different from results for 100% saturation), but it is necessary to check if the calculations in js and here are matching.  

See details in code comments.  

