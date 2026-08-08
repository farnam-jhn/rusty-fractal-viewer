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
    fn iterations_count(&self, point: Complex<f64>) -> i32;
    fn get_name() -> String;
    fn get_centerx(&self) -> f64;
}

// -- Mandelbrot Set --
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

    fn iterations_count(&self, point: Complex<f64>) -> i32 {
        let mut z: Complex<f64> = Complex { re: 0.0, im: 0.0 };
        let mut iterations: i32 = 0;

        let threshold: f64 = self.config.max_magnitude * self.config.max_magnitude;

        while iterations < self.config.max_iterations {
            z = Complex {
                re: (z.re * z.re - z.im * z.im + point.re),
                im: (2.0 * z.re * z.im + point.im),
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
// -- Julia Set --
pub struct Julia {
    constant: Complex<f64>,
    centerx: f64,
    config: FractalConfig,
}

impl Julia {
    pub fn new(max_iterations: i32, max_magnitude: f64, constant: Complex<f64>) -> Self {
        Self {
            constant: constant,
            centerx: 0.0,
            config: FractalConfig {
                max_iterations,
                max_magnitude,
            },
        }
    }
}

impl Fractal for Julia {
    fn config(&self) -> &FractalConfig {
        &self.config
    }

    fn config_mut(&mut self) -> &mut FractalConfig {
        &mut self.config
    }

    fn iterations_count(&self, point: Complex<f64>) -> i32 {
        let mut iterations: i32 = 0;
        let mut z = point;

        let threshold: f64 = self.config.max_magnitude * self.config.max_magnitude;

        while iterations < self.config.max_iterations {
            z = Complex {
                re: (z.re * z.re - z.im * z.im + self.constant.re),
                im: (2.0 * z.re * z.im + self.constant.im),
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
// -- Burning Ship --
pub struct BurningShip {
    centerx: f64,
    config: FractalConfig,
}

impl BurningShip {
    pub fn new(max_iterations: i32, max_magnitude: f64) -> Self {
        Self {
            centerx: 0.0,
            config: FractalConfig {
                max_iterations,
                max_magnitude,
            },
        }
    }
}

impl Fractal for BurningShip {
    fn config(&self) -> &FractalConfig {
        &self.config
    }

    fn config_mut(&mut self) -> &mut FractalConfig {
        &mut self.config
    }

    fn iterations_count(&self, point: Complex<f64>) -> i32 {
        let mut z: Complex<f64> = Complex { re: 0.0, im: 0.0 };
        let mut iterations: i32 = 0;

        let threshold: f64 = self.config.max_magnitude * self.config.max_magnitude;

        while iterations < self.config.max_iterations {
            let temp = Complex::new(z.re.abs(), z.im.abs());
            z = temp * temp + point;

            if z.re * z.re + z.im * z.im > threshold {
                break;
            }

            iterations += 1;
        }

        return iterations;
    }

    fn get_name() -> String {
        format!("BurningShip")
    }

    fn get_centerx(&self) -> f64 {
        self.centerx
    }
}
// -- Tricorn --
pub struct Tricorn {
    centerx: f64,
    config: FractalConfig,
}

impl Tricorn {
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

impl Fractal for Tricorn {
    fn config(&self) -> &FractalConfig {
        &self.config
    }

    fn config_mut(&mut self) -> &mut FractalConfig {
        &mut self.config
    }

    fn iterations_count(&self, point: Complex<f64>) -> i32 {
        let mut z: Complex<f64> = Complex { re: 0.0, im: 0.0 };
        let mut iterations: i32 = 0;

        let threshold: f64 = self.config.max_magnitude * self.config.max_magnitude;

        while iterations < self.config.max_iterations {
            let temp = z.conj();

            z = temp * temp + point;

            if z.re * z.re + z.im * z.im > threshold {
                break;
            }

            iterations += 1;
        }

        return iterations;
    }

    fn get_name() -> String {
        format!("Tricorn")
    }

    fn get_centerx(&self) -> f64 {
        self.centerx
    }
}
