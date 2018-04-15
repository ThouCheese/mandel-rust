mod imperative_mandel;
mod functional_mandel;

fn main() {
    let _result = imperative_mandel::mandelbrot(5000, 5000, -1.5, 1.5, -2.0, 1.0);
}
