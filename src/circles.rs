#[cfg(feature = "vec")]
use svg::node::element;

#[cfg(any(feature = "vec", feature = "pix"))]
use crate::colors::Color;

/// Information about the circle
#[cfg(feature = "pix")]
#[derive(Clone, Copy, Debug, PartialEq)]
struct Circle {
    x_center: f32,
    y_center: f32,
    radius: f32,
    rgba_color: Color,
}

/// Function to determine if the point (x, y) is within the circle
#[cfg(feature = "pix")]
fn in_circle(x: i32, y: i32, circle: &Circle) -> bool {
    (x as f32 - circle.x_center).powi(2) + (y as f32 - circle.y_center).powi(2)
        < circle.radius.powi(2)
}

/// Information about circle center position
///
/// `position_circle_set` sets default positions for small circles in 19-circles icon 
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CirclePosition {
    pub x_center: f32,
    pub y_center: f32,
}

/// Set default positions of small circles in 19-circles icon
///
/// Input is `f32` center-to-center distance between small circles
pub fn position_circle_set(center_to_center: f32) -> [CirclePosition; 19] {
    let a = center_to_center;
    let b = center_to_center * 3f32.sqrt() / 2f32;
    [
        CirclePosition {
            x_center: 0f32,
            y_center: -2f32 * a,
        },
        CirclePosition {
            x_center: 0f32,
            y_center: -a,
        },
        CirclePosition {
            x_center: -b,
            y_center: -3f32 * a / 2f32,
        },
        CirclePosition {
            x_center: -2f32 * b,
            y_center: -a,
        },
        CirclePosition {
            x_center: -b,
            y_center: -a / 2f32,
        },
        CirclePosition {
            x_center: -2f32 * b,
            y_center: 0f32,
        },
        CirclePosition {
            x_center: -2f32 * b,
            y_center: a,
        },
        CirclePosition {
            x_center: -b,
            y_center: a / 2f32,
        },
        CirclePosition {
            x_center: -b,
            y_center: 3f32 * a / 2f32,
        },
        CirclePosition {
            x_center: 0f32,
            y_center: 2f32 * a,
        },
        CirclePosition {
            x_center: 0f32,
            y_center: a,
        },
        CirclePosition {
            x_center: b,
            y_center: 3f32 * a / 2f32,
        },
        CirclePosition {
            x_center: 2f32 * b,
            y_center: a,
        },
        CirclePosition {
            x_center: b,
            y_center: a / 2f32,
        },
        CirclePosition {
            x_center: 2f32 * b,
            y_center: 0f32,
        },
        CirclePosition {
            x_center: 2f32 * b,
            y_center: -a,
        },
        CirclePosition {
            x_center: b,
            y_center: -a / 2f32,
        },
        CirclePosition {
            x_center: b,
            y_center: -3f32 * a / 2f32,
        },
        CirclePosition {
            x_center: 0f32,
            y_center: 0f32,
        },
    ]
}

/// Function to finalize 19 circles with properly corresponding colors and radius
#[cfg(feature = "pix")]
fn get_colored_circles(
    center_to_center: f32,
    small_radius: f32,
    colors: [Color; 19],
) -> [Circle; 19] {
    let positions = position_circle_set(center_to_center);
    let mut out: Vec<Circle> = Vec::with_capacity(19);
    for (i, position) in positions.iter().enumerate() {
        let new = Circle {
            x_center: position.x_center,
            y_center: position.y_center,
            radius: small_radius,
            rgba_color: colors[i],
        };
        out.push(new);
    }
    out.try_into().expect("always generate 19-element set")
}

/// Calculate `png` image pixel data (only pixel colors)
///
/// Iterates over all `png` image pixels and sets the color.
///
/// Requires image size in pixels (equal to diameter of largest, outer circle),
/// and identicon colors
#[cfg(feature = "pix")]
pub fn calculate_png_data(size_in_pixels: u16, colors: [Color; 19]) -> Vec<u8> {
    let mut data: Vec<u8> = Vec::new();
    let big_radius = size_in_pixels as f32 / 2f32;
    let small_radius = big_radius / 32f32 * 5f32;
    let center_to_center = big_radius / 8f32 * 3f32;

    let big_circle = Circle {
        x_center: 0f32,
        y_center: 0f32,
        radius: big_radius,
        rgba_color: Color::foreground(),
    };

    let small_circles_set = get_colored_circles(center_to_center, small_radius, colors);

    let iter_start = -(size_in_pixels as i32) / 2;
    let iter_end = {
        (size_in_pixels >> 1) + (size_in_pixels & 0x01)
    } as i32;

    // calculating color for each pixel
    for y in iter_start..iter_end {
        for x in iter_start..iter_end {
            if in_circle(x, y, &big_circle) {
                let mut some_small_circle = None;
                for cir in small_circles_set.iter() {
                    if in_circle(x, y, cir) {
                        some_small_circle = Some(cir.rgba_color);
                        break;
                    }
                }
                match some_small_circle {
                    Some(color) => data.extend_from_slice(&color.to_slice()),
                    None => data.extend_from_slice(&big_circle.rgba_color.to_slice()),
                }
            } else {
                data.extend_from_slice(&Color::background().to_slice())
            }
        }
    }
    data
}

/// Calculate `svg` file contents
///
/// Inputs radius of outer circle (largest one) and identicon colors
#[cfg(feature = "vec")]
pub fn calculate_svg_data(big_radius: f32, colors: [Color; 19]) -> Vec<element::Circle> {
    let mut out: Vec<element::Circle> = Vec::with_capacity(20);
    out.push(
        element::Circle::new()
            .set("cx", 0f32)
            .set("cy", 0f32)
            .set("r", big_radius)
            .set("fill", Color::foreground().to_hex())
            .set("stroke", "none"),
    );
    let small_radius = big_radius / 32f32 * 5f32;
    let center_to_center = big_radius / 8f32 * 3f32;
    let positions = position_circle_set(center_to_center);
    for (i, position) in positions.iter().enumerate() {
        out.push(
            element::Circle::new()
                .set("cx", position.x_center)
                .set("cy", position.y_center)
                .set("r", small_radius)
                .set("fill", colors[i].to_hex())
                .set("stroke", "none"),
        );
    }
    out
}
