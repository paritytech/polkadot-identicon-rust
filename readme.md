
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

