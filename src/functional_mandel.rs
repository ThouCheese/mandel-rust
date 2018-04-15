extern crate rayon;

use par::rayon::prelude::*;

static MAX_ITERATIONS: u16 = 500;

type Precision = f64;

fn map_pixel(pixel: usize, pixel_count: usize,
             image_lower_bound: Precision, image_upper_bound: Precision) -> Precision {
    (image_upper_bound - image_lower_bound)
        / pixel_count as Precision * pixel as Precision + image_lower_bound
}

pub fn mandelbrot(pixel_height: usize, pixel_width: usize,
               complex_lower: Precision, complex_higher: Precision,
               complex_left: Precision, complex_right: Precision) -> Vec<Vec<u16>> {
    let range: Vec<usize> = (0..pixel_height).collect();
    let result: Vec<Vec<u16>> = range.iter().map(|i| {
        let a = map_pixel(*i, pixel_width, complex_left, complex_right);
        (0..pixel_width).map(|j| {
            let b = map_pixel(j, pixel_height, complex_lower, complex_higher);
            let mut z_real = a * a + b * b;
            let mut z_imag = 2 as Precision * a * b;
            let mut z_real_squared = z_real * z_real;
            let mut z_imag_squared = z_imag * z_imag;
            let mut iter: u16 = 0;
            while z_real_squared + z_imag_squared < 4 as Precision &&
                iter < MAX_ITERATIONS {
                z_imag = 2 as Precision * z_real * z_imag + b;
                z_real = z_real_squared - z_imag_squared + a;
                z_real_squared = z_real * z_real;
                z_imag_squared = z_imag * z_imag;
                iter += 1;
            }
            iter
        }).collect()
    }).collect();
    result
}
