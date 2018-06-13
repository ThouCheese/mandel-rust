#![feature(test)]
#![allow(dead_code)]

extern crate test;
extern crate bmp;

mod mandel;

const SIZE: usize = 10_000;
const PIXEL_RATIO: f64 = 255f64 / mandel::MAX_ITERATIONS as f64;

fn map_colour(iter: u16) -> u16 {
	if iter == mandel::MAX_ITERATIONS {
		0
	} else if iter < mandel::MAX_ITERATIONS / 4 {
		2 * iter
	} else {
		mandel::MAX_ITERATIONS / 2 + iter / 2
	}
}

fn main() -> Result<(), std::io::Error> {
	println!("gnaak");
	let mut bitmap = bmp::Image::new(SIZE as u32, SIZE as u32);
    let mb = mandel::mandelbrot(SIZE, SIZE, -1.5, 1.5, -2.0, 1.0);
    for (row_index, row) in mb.into_iter().enumerate() {
    	for (column_index, iter_count) in row.into_iter().enumerate() {
    		let iter_count = (f64::from(map_colour(iter_count)) * PIXEL_RATIO) as u8;
			bitmap.set_pixel(row_index as u32,
						     column_index as u32, 
						     bmp::Pixel::new(iter_count, iter_count, iter_count));
    	}
    }
    bitmap.save("image.bmp")?;
    Ok(())
}

#[cfg(test)]
mod benches {
	use super::*;
	use test::Bencher;

	#[bench]
	fn bench_mandel(b: &mut Bencher) {
		b.iter(|| mandel::mandelbrot(1000, 1000, -1.5, 1.5, -2.0, 1.0));
	}

	#[bench]
	fn bench_mandel3(b: &mut Bencher) {
		b.iter(|| mandel::mandelbrot3(1000, 1000, -1.5, 1.5, -2.0, 1.0));
	}
}
