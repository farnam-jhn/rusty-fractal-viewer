In [The math behind](obsidian://open?vault=rusty-fractal-viewer&file=The%20math%20behind), we saw we need to check if a recursive complex sequence converges or escapes to infinity.
Because infinite iterations are impossible in computing, we use finite iteration limits and magnitude thresholds.

### Approach
To check if a complex coordinate $c$ escapes:
1. **Magnitude Bounding Check**: Test if $|Z(n)|$ exceeds a maximum threshold (e.g., threshold magnitude $R = 2.0$). Once $|Z| \ge 2$, sequence divergence is guaranteed.
2. **Iteration Count Limit**: Halt computation after a maximum number of iterations (`max_iterations`). Points reaching this limit are inferred to be within the set.

#### Optimization: Avoiding Square Roots
Instead of calculating the actual Euclidean magnitude $|Z| = \sqrt{\text{re}^2 + \text{im}^2}$, we compare the norm (squared magnitude) against the squared threshold ($R^2 = 4.0$):

```rust
let threshold = self.config.max_magnitude * self.config.max_magnitude;
if z.re * z.re + z.im * z.im > threshold {
    break;
}
```
This avoids performing square root calculations inside millions of pixel rendering loops per frame.

### Rust Trait Architecture
To maintain an extensible and modular architecture, the viewer defines a `Fractal` trait implemented by all set types (`Mandelbrot`, `Julia`, `BurningShip`, `Tricorn`):

```rust
pub trait Fractal {
    fn config(&self) -> &FractalConfig;
    fn config_mut(&mut self) -> &mut FractalConfig;
    fn iterations_count(&self, point: Complex<f64>) -> i32;
    fn get_name() -> String;
    fn get_centerx(&self) -> f64;
}
```
This allows clean polymorphic execution of escape-time checks across different fractal variants.

### Viewport Transformation & Interactivity
Screen coordinates are translated to complex coordinates via a flexible `Viewport` struct:
- **Aspect Ratio Correction**: Adjusts span dynamically based on window resolution.
- **Mouse-Anchored Zooming (`zoom_at`)**: Preserves the complex coordinate beneath the cursor during wheel zoom.
- **Panning (`pan`)**: Updates coordinate origin dynamically when dragging.

### Color Palettes & Linear Interpolation (Lerp)
To produce smooth gradient visualizations from iteration counts, discrete counts are mapped continuously using color interpolation (`lerp_color`):

```rust
pub fn lerp_color(color1: Color, color2: Color, t: f64) -> Color {
    let lerp = |x: f32, y: f32| -> f32 {
        let x_u8 = (x * 255.0) as f64;
        let y_u8 = (y * 255.0) as f64;
        let res_u8 = (x_u8 + (y_u8 - x_u8) * t) as u8;
        res_u8 as f32 / 255.0
    };
    ...
}
```
`smooth_color` scales iteration counts across multi-stop color palettes (`Palette::classic()`), providing smooth color transitions.

### Graphics & Performance Stack
- **`macroquad`**: Provides windowing, texture management, and fast CPU pixel buffer updates (`Texture2D`).
- **`num-complex`**: Provides double-precision `Complex<f64>` calculations.
- **Memory Safety & Zero-Cost Abstractions**: Eliminates manual pointer management and memory bugs while retaining native performance.
