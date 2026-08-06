use num_complex::{self, Complex};

pub struct FractalConfig {
    pub max_iterations: i32,
    max_magnitude: f64,
}

impl Default for FractalConfig {
    fn default() -> Self {
        Self {
            max_iterations: 200,
            max_magnitude: 2.0,
        }
    }
}

pub trait Fractal {
    fn config(&self) -> &FractalConfig;
    fn config_mut(&mut self) -> &mut FractalConfig;
    fn iterations_count(&self, c: Complex<f64>) -> i32;
    fn get_name() -> String;
    fn get_centerx(&self) -> f64;
}

pub struct Mandelbrot {
    centerx: f64,
    config: FractalConfig,
}

impl Mandelbrot {
    pub fn new(max_iterations: i32, max_magnitude: f64) -> Self {
        Self {
            centerx: -0.5,
            config: FractalConfig {
                max_iterations,
                max_magnitude,
            },
        }
    }
}

impl Fractal for Mandelbrot {
    fn config(&self) -> &FractalConfig {
        &self.config
    }

    fn config_mut(&mut self) -> &mut FractalConfig {
        &mut self.config
    }

    fn iterations_count(&self, c: Complex<f64>) -> i32 {
        let mut z: Complex<f64> = Complex { re: 0.0, im: 0.0 };
        let mut iterations: i32 = 0;

        let threshold: f64 = self.config.max_magnitude * self.config.max_magnitude;

        while iterations < self.config.max_iterations {
            z = Complex {
                re: (z.re * z.re - z.im * z.im + c.re),
                im: (2.0 * z.re * z.im + c.im),
            };

            if z.re * z.re + z.im * z.im > threshold {
                break;
            }

            iterations += 1;
        }

        return iterations;
    }

    fn get_name() -> String {
        format!("Mandelbrot")
    }

    fn get_centerx(&self) -> f64 {
        self.centerx
    }
}
