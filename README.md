
# Crate `plot_icon`

## Overview

This is a lib crate for generating standard 19-circle icons in png and in svg format.  

Output could be in form of .png or .svg files, or the Vec<u8> with png data, or svg::Document with svg data.  

The code is following the published javascript code for icon generation from `https://github.com/paritytech/oo7/blob/master/packages/polkadot-identicon/src/index.jsx`.  



## Input needed to generate data (to be used in Signer)  

Input requires:  

1. actual input data:  
    - `Vec<u8>` to be used with `png_data_from_vec` and `svg_from_vec` functions  
    - or hex format string to be used with `png_data_from_hex` and `svg_from_hex` functions  
    - or base58 format string to be used with `png_data_from_base58` and `svg_from_base58` functions  
2. size parameter `halfsize` of final file (created image is square, with ``halfsize`*2+1`` pixels side for png and ``halfsize`*2`` for svg)  


## Input needed to make files (at the moment, mainly for test pics)  

Input requires:  

1. actual input data:  
    - `Vec<u8>` to be used with `plot_png_from_vec` and `plot_svg_from_vec` functions  
    - or hex format string to be used with `plot_png_from_hex` and `plot_svg_from_hex` functions  
    - or base58 format string to be used with `plot_png_from_base58` and `plot_svg_from_base58` functions  
2. size parameter `halfsize` of final file (created image is square, with ``halfsize`*2+1`` pixels side for png and ``halfsize`*2`` for svg)  
3. output file name  


## Tests and Examples

Currently there are tests to check if the color sets calculated for Alice and Bob are identical to the colors in the corresponding well-known icons.  

Examples are plotting both .png and .svg icons for base58 westend Alice address, and hex westend genesis hash.


## Notes

There are several uncertainties about how the original published code was designed to work, those should be clarified.  

For example, calculated HSL color saturation could range 30..109, and is processed as percents. Crate `palette` (currently used here) processes saturation values over 100 as percents over 100, and gives some results (slightly different from results for 100% saturation), but it is necessary to check if the calculations in js and here are matching.  

See details in code comments.  


## Small identicons  

Signer uses images in .png format, since .svg format is not sufficiently supported on devices side. Hopefully this change at some point, however, for the time being .svg remains an optional feature, and this crate sticks mostly to .png generation.  
For use in Signer, the .png identicons produced should be, obviously, quite small (at the moment, 16 pix halfsize and device-independent).  

As .png images are generated pixel-by-pixel, each pixel color is decided based on calculations in integer values, and in small-sized images, such as 16 pix target halfsize, the image main features (small colored circles) disappear entirely.  

One possible solution to this is to calculate in floats, and only then transform into integers. With small identicons the images have the colored dots, but are strongly pixelated.  

Possible solution is to generate larger identicon and then scale it down in frontend, but it was noticed that the scaling results (pixelation, color even distribution) are device-dependent and although a minor thing, it should definitely be avoided in *identicon*.  

To keep the rescaling within this crate, a larger .png is generated (scaling factor could be set up by user, default is 8), and then scaled down to original desired size using `image` crate function `resize` with a filter ([`FilterType`](https://docs.rs/image/latest/image/imageops/enum.FilterType.html) could be set up by user, default is CatmullRom). All filters produce reasonable results, except `FilterType::Nearest` that results in substantially distorted image and is therefore not recommended.  

The scaling approach seems to make some visible difference only for small identicon pictures (below 100 pix halfsize). For larger identicons `png_data` produces reasonable results without rescaling tricks.  

To generate the small identicon with default scaling parameters, use `png_data_scaled_default`. It inputs only `&[u8]` data (the one that should be drawn as identicon) and image halfsize in pixels. It outputs png data as `Vec<u8>`. Printing the data into .png file could be done `std::fs::write` (see examples).  

To generate the small identicon with custom scaling parameters, use `png_data_scaled_custom`, with custom scaling factor and filter type (see examples).  

Another issue, especially pronounced for small png images, is off-centering in images with even number of pixels sides. Rescaling compensates for this effect greatly.  
