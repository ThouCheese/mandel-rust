extern crate rayon;

use mandel::rayon::iter::{IntoParallelIterator, ParallelIterator};

type Iterations = u16;
type Precision = f64;

pub const MAX_ITERATIONS: Iterations = 100;

/// maps a width or height in pixels to a real of imaginary part of a complex number
#[inline]
fn map_pixel(pixel: usize, pixel_count: usize,
             image_lower_bound: Precision, image_upper_bound: Precision) -> Precision {
    (image_upper_bound - image_lower_bound)
        / pixel_count as Precision * pixel as Precision + image_lower_bound
}

pub fn mandelbrot(pixel_height: usize, pixel_width: usize,
                  complex_lower: Precision, complex_higher: Precision,
                  complex_left: Precision, complex_right: Precision) -> Vec<Vec<Iterations>> {
    (0..pixel_height).into_par_iter().map(|i| {
        let a = map_pixel(i, pixel_width, complex_left, complex_right);
        (0..pixel_width).map(|j| {
            let b = map_pixel(j, pixel_height, complex_lower, complex_higher);
            let mut z_real = a * a - b * b + a;
            let mut z_imag = Precision::from(2) * a * b + b;
            let mut z_real_squared = z_real * z_real;
            let mut z_imag_squared = z_imag * z_imag;
            let mut iter: Iterations = 0;
            while z_real_squared + z_imag_squared < Precision::from(4) && iter < MAX_ITERATIONS {
                z_imag = Precision::from(2) * z_real * z_imag + b;
                z_real = z_real_squared - z_imag_squared + a;
                z_real_squared = z_real * z_real;
                z_imag_squared = z_imag * z_imag;
                iter += 1;
            }
            iter
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>()
}

pub fn mandelbrot3(pixel_height: usize, pixel_width: usize,
                  complex_lower: Precision, complex_higher: Precision,
                  complex_left: Precision, complex_right: Precision) -> Vec<Vec<Iterations>> {
    (0..pixel_height).into_par_iter().map(|i| {
        let a = map_pixel(i, pixel_width, complex_left, complex_right);
        (0..pixel_width).map(|j| {
            let b = map_pixel(j, pixel_height, complex_lower, complex_higher);
            let mut r = 0f64;
            let mut i = 0f64;
            let (mut r2, mut i2) = (r * r, i * i);
            let (mut r3, mut i3) = (r2 * r, i2 * i);
            let mut iter: Iterations = 0;
            while r2 + i2 < Precision::from(4) && iter < MAX_ITERATIONS {
                i = Precision::from(3) * r2 * i - i3 + b;
                r = r3 - Precision::from(3) * r * i2 + a;
                r2 = r * r;
                i2 = i * i;
                r3 = r2 * r;
                i3 = i2 * i;
                iter += 1;
            }
            iter
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>()
}
