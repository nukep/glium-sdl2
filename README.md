# glium_sdl2

[![Build Status](https://travis-ci.org/nukep/glium-sdl2.svg)](https://travis-ci.org/nukep/glium-sdl2)
[![Latest Version](https://img.shields.io/crates/v/glium_sdl2.svg)](https://crates.io/crates/glium_sdl2)

An SDL2 backend for [Glium](https://github.com/tomaka/glium) - a high-level
OpenGL wrapper for the Rust language.

This library, along with `glium` and [`rust-sdl2`](https://github.com/Rust-SDL2/rust-sdl2),
are in heavy development and are subject to change.

```toml
[dependencies]
glium_sdl2 = "0.15"
sdl2 = "0.30"
glium = "0.18"

features = []
default-features = false
```

glium_sdl2 doesn't reexport the `glium` or `sdl2` crates, so you must declare
them _with the versions listed above_ in your `Cargo.toml` file.

glium_sdl2's version will be bumped once this library, `glium` or `sdl2`
make breaking changes.

## [Documentation](https://docs.rs/glium_sdl2)

## [Change log](CHANGELOG.md)

## Example usage

See the examples/ folder for examples. Here's a bare-bones skeleton program that initializes SDL2 and Glium, and does nothing:
* [blank.rs](examples/blank.rs)

Using glium with SDL2 is very similar to using glium with glutin.
Generally speaking, Glium documentation and tutorials should be fairly trivial
to adapt to SDL2.

## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.
