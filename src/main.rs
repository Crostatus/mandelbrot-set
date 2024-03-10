mod tests;

use num::{complex::ComplexFloat, Complex};
use std::str::FromStr;

fn main() {
    println!("Hello, world!");
}

fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None,
    }
}

/// Parse the string 's' as a coordinate pair, like "1920x1080"
/// 's' should have <T><sep><T>
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        Some(sep_index) => match (
            T::from_str(&s[..sep_index]),
            T::from_str(&s[sep_index + 1..]),
        ) {
            (Ok(left), Ok(right)) => Some((left, right)),
            _ => None,
        },
        None => None,
    }
}

/// Try to determine if 'c' is in the Mandlbrot set, using at most 'limit' iterations to decide.
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }

    None
}

/// Maps a pixel pair to the corresponding point on the complex plane
///
/// 'bounds' is a pair giving the width and height of the image in pixels.
/// 'pixel' pixel coordinates in that image.
/// 'upper_left' and 'lower_right' parameters are points on the complex plane
fn pixel_to_complex_plain(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );

    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
    }
}

/// Render a rectangle of the Mandelbrot set into a buffer of pixels
fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    assert!(pixels.len() == bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_complex_plain(bounds, (column, row), upper_left, lower_right);

            pixels[row * bounds.0 + column] = match escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8,
            };
        }
    }
}
