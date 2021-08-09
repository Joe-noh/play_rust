use num_complex::Complex;

pub struct Mandelbrot {
    rows: Vec<Vec<i32>>,
}

impl Mandelbrot {
    pub fn generate<'a>(width: i32, height: i32) -> Self {
        let x_min = -2.0;
        let x_max = 2.0;
        let y_min = -1.0;
        let y_max = 1.0;

        let mut rows: Vec<_> = Vec::with_capacity(width as usize);

        for img_y in 0..height {
            let mut row: Vec<_> = Vec::with_capacity(height as usize);

            for img_x in 0..width {
                let x_percent = (img_x as f64) / (width as f64);
                let y_percent = (img_y as f64) / (height as f64);
                let cx = x_min + (x_max - x_min) * x_percent;
                let cy = y_min + (y_max - y_min) * y_percent;

                row.push(Self::point(cx, cy));
            }

            rows.push(row);
        }

        Mandelbrot { rows: rows }
    }

    fn point(cx: f64, cy: f64) -> i32 {
        let max_iter = 1000;
        let mut z = Complex::new(0.0, 0.0);
        let c = Complex::new(cx, cy);

        for i in 0..max_iter {
            if z.norm() > 2.0 {
                return i
            }
            z = z * z + c;
        }

        max_iter
    }

    pub fn render(&self) {
        for row in self.rows.iter() {
            let mut line = String::with_capacity(row.len());

            for column in row {
                let val = match column {
                    0..=2 => ' ',
                    3..=5 => '.',
                    6..=10 => '+',
                    11..=30 => '*',
                    31..=100 => '$',
                    101..=200 => '&',
                    201..=400 => '@',
                    401..=700 => '#',
                    _ => '%'
                };

                line.push(val);
            }

            println!("{}", line);
        }
    }
}
