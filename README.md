# Rusty Fractal Viewer
Rusty fractal viewer is a fractal viewer (only fractals that can be computed using escape time algorithm) written in rust.  
[![Conventional Commit](https://img.shields.io/badge/Conventional%20Commits-1.0.0-green)](https://www.conventionalcommits.org/en/v1.0.0/) ![License](https://img.shields.io/badge/License-GNU_GPLv3-0052cc?logo=license)

---

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
<img src=docs/resources/scv1.png width="40%">

## Docs
Detailed documentation for this project is available in the [`docs/`](docs/) folder:
- [Fractal viewer index](docs/Fractal%20viewer.md)
- [The math behind](docs/The%20math%20behind.md)
- [Rust Implementation](docs/Rust%20Implementation.md)
- [Versions](docs/Versions.md)
- [TODOs](docs/TODOs.md)

## License
This project is licensed under GNU GPLv3. For more info check out the [License file](LICENSE).
