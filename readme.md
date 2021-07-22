
# Crate `plot_icon`

## Overview

This is a crate for generating standard 19-circle icons in png format.  

The code is following the published javascript code for icon generation from `https://github.com/paritytech/oo7/blob/master/packages/polkadot-identicon/src/index.jsx`.  

Currently the crate is operated through adjusting `main.rs`, and plots png file as successful output. This could be modified if needed.  


## Input

Input requires:  

1. actual input data:  
    - `Vec<u8>` to be used with `plot_icon_from_vec` function  
    - or hex format string to be used with `plot_icon_from_hex` function  
    - or base58 format string to be used with `plot_icon_from_base58` function  
2. size parameter `halfsize_in_pixels` of final file (created image is square, with `halfsize_in_pixels`*2+1 side)  
3. output file name  


## Tests

Currently there are tests to check if the color sets calculated for Alice and Bob are identical to the colors in the corresponding well-known icons.  


## Notes

There are several uncertainties about how the original published code was designed to work, those should be clarified.  

For example, calculated HSL color saturation could range 30..109, and is processed as percents. Crate `palette` (currently used here) processes saturation values over 100 as percents over 100, and gives some results (slightly different from results for 100% saturation), but it is necessary to check if the calculations in js and here are matching.  

See details in code comments.  

