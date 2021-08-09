mod mandelbrot;

fn main() {
    let m = mandelbrot::Mandelbrot::generate(180, 40);
    m.render();
}
