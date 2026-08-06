# Rusty Fractal Viewer
Rusty fractal viewer is a fractal viewer (only fractals that can be computed using escape time algorithm) written in rust.

## Origin
Before start of this project, i wrote the same thing in C++ using SFML Game engine, i archived the [old repo](https://github.com/farnam-jhn/FractalViewer) and started development of this project for two reasons, first is the meomory saftey and the second one is learning rust and dealing with the question "Why Rust?". 

## Tech stack
| **Field**         | **Technology**                                                             |
|-------------------|----------------------------------------------------------------------------|
| **Main Language** | ![Rust](https://shields.io/badge/Rust-black?style=for-the-badge&logo=rust) |
| **Engine**        | ![Macroquad](https://shields.io/badge/Macroquad-black?style=for-the-badge) |
## Build & Run
To build the project:
```bash
git clone https://github.com/farnam-jhn/rusty-fractal-viewer.git
cd rusty-fractal-viewer
cargo build --release
```
and to run it:
```bash
./target/release/rusty-fractal-viewer
```

## Gallery
![v1](docs/resources/scv1.png)

## Docs
Documentations are available at the [old repo](https://github.com/farnam-jhn/FractalViewer), at this point no other documentation is needed but later on i would add doumentation is needed.