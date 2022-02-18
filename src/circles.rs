use svg::node::element;

const BACKGROUND_COLOR: [u8; 4] = [255, 255, 255, 0];
pub const FOREGROUND_COLOR: [u8; 4] = [238, 238, 238, 255];

/// Struct to store information about the circle
struct Circle {
    x_center: f64,
    y_center: f64,
    radius: f64,
    rgba_color: [u8; 4],
}


/// Function to determine if the point (x, y) is within the circle
fn in_circle (x: i32, y: i32, circle: &Circle) -> bool {
    (x as f64 - circle.x_center).powf(2.0) + (y as f64 - circle.y_center).powf(2.0) < circle.radius.powf(2.0)
}

/// Struct to store information about circle center position
/// For 19-circle icons circle positions are set as defaults
struct CirclePosition {
    x_center: f64,
    y_center: f64,
}

/// Function to set default positions of small circles in 19-circle icon
/// a is center_to_center distance
fn position_circle_set (a: f64) -> Vec<CirclePosition> {

    let a = a as f64;
    let b = a * 3f64.sqrt()/2.0;
    vec! [
        CirclePosition {
            x_center: 0.0,
            y_center: -2.0*a
        },
        CirclePosition {
            x_center: 0.0,
            y_center: -a
        },
        CirclePosition {
            x_center: -b,
            y_center: -3.0*a/2.0
        },
        CirclePosition {
            x_center: -2.0*b,
            y_center: -a
        },
        CirclePosition {
            x_center: -b,
            y_center: -a/2.0
        },
        CirclePosition {
            x_center: -2.0*b,
            y_center: 0.0
        },
        CirclePosition {
            x_center: -2.0*b,
            y_center: a
        },
        CirclePosition {
            x_center: -b,
            y_center: a/2.0
        },
        CirclePosition {
            x_center: -b,
            y_center: 3.0*a/2.0
        },
        CirclePosition {
            x_center: 0.0,
            y_center: 2.0*a
        },
        CirclePosition {
            x_center: 0.0,
            y_center: a
        },
        CirclePosition {
            x_center: b,
            y_center: 3.0*a/2.0
        },
        CirclePosition {
            x_center: 2.0*b,
            y_center: a
        },
        CirclePosition {
            x_center: b,
            y_center: a/2.0
        },
        CirclePosition {
            x_center: 2.0*b,
            y_center: 0.0
        },
        CirclePosition {
            x_center: 2.0*b,
            y_center: -a
        },
        CirclePosition {
            x_center: b,
            y_center: -a/2.0
        },
        CirclePosition {
            x_center: b,
            y_center: -3.0*a/2.0
        },
        CirclePosition {
            x_center: 0.0,
            y_center: 0.0
        }
    ]
}

/// function to finalize 19 circles with properly corresponding colors and radius
fn get_colored_circles (center_to_center: f64, small_radius: f64, colors: Vec<[u8; 4]>) -> Vec<Circle> {
    let positions = position_circle_set(center_to_center);
    let mut out: Vec<Circle> = Vec::with_capacity(19);
// no checking is done here for positions.len() == 19 and colors.len() == 19;
// however, no other length is expected.
    for (i, position) in positions.iter().enumerate() {
        let new = Circle {
            x_center: position.x_center,
            y_center: position.y_center,
            radius: small_radius,
            rgba_color: colors[i],
        };
        out.push(new);
    }
    out
}

/// function to calculate contents of the png image with 19-circle icon
pub fn calculate_png_data (big_radius: i32, colors: Vec<[u8; 4]>) -> Vec<u8> {
    
    let mut data: Vec<u8> = Vec::new();
    let small_radius = big_radius as f64/32.0*5.0;
    let center_to_center = big_radius as f64/8.0*3.0;
    
    let big_circle = Circle {
        x_center: 0.0,
        y_center: 0.0,
        radius: big_radius as f64,
        rgba_color: FOREGROUND_COLOR,
    };
    
    let small_circles_set = get_colored_circles (center_to_center, small_radius, colors);
    
    for y in -big_radius..big_radius+1 {
        for x in -big_radius..big_radius+1 {
            if in_circle (x, y, &big_circle) {
                let mut some_small_circle = None;
                for cir in small_circles_set.iter() {
                    if in_circle(x, y, cir) {
                        some_small_circle = Some(cir.rgba_color);
                        break;
                    }
                }
                match some_small_circle {
                    Some(color) => data.extend_from_slice(&color),
                    None => data.extend_from_slice(&big_circle.rgba_color)
                }
            }
            else {data.extend_from_slice(&BACKGROUND_COLOR)}
        }
    }
    data
}


/// Function to calculate svg file contents (using element::Circle from svg crate)
pub fn calculate_svg_data (big_radius: i32, colors: Vec<[u8; 4]>) -> Vec<element::Circle> {
    
    let big_radius = big_radius as f64;
    let mut out: Vec<element::Circle> = Vec::with_capacity(20);
    out.push(element::Circle::new()
        .set("cx", 0.0)
        .set("cy", 0.0)
        .set("r", big_radius)
        .set("fill", rgba_to_hex(FOREGROUND_COLOR))
        .set("stroke", "none")
    );
    let small_radius = big_radius/32.0*5.0;
    let center_to_center = big_radius/8.0*3.0;
    let positions = position_circle_set(center_to_center);
    for (i, position) in positions.iter().enumerate() {
        out.push(element::Circle::new()
            .set("cx", position.x_center)
            .set("cy", position.y_center)
            .set("r", small_radius)
            .set("fill", rgba_to_hex(colors[i]))
            .set("stroke", "none")
        );
    }
    out
}


/// Helper function to transform RGBA [u8; 4] color needed for png into 
/// hex string color needed for svg
fn rgba_to_hex (rgba_color: [u8; 4]) -> String {
    format!("#{}", hex::encode(vec![rgba_color[0], rgba_color[1], rgba_color[2]]))
}
